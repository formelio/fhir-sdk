//! FHIR search paging functionality

use std::{collections::VecDeque, pin::Pin, task::Poll};

use fhir_model::stu3::{
	codes::{BundleType, SearchEntryMode},
	resources::{Bundle, BundleEntry, DomainResource, NamedResource, Resource, },
};
use futures::{future::BoxFuture, ready, Future, FutureExt, Stream, StreamExt};
use reqwest::Url;

use crate::client::{stu3::references::populate_reference_targets_internal, search::Paged};

use super::{find_next_page_url, Client, Error, FhirStu3};

/// Unwraps search pages into a single stream of resources. The resources
/// can be consumed via the `Stream`/`StreamExt` traits.
pub struct Unpaged<R> {
	/// The FHIR client to make further requests for the next pages.
	client: Client<FhirStu3>,
	/// The current page of matches.
	page: Page<R>,
	/// Current future to retrieve the next page.
	future_next_page: Option<BoxFuture<'static, Result<Page<R>, Error>>>,
}

impl<R> Unpaged<R>
where
	R: NamedResource + DomainResource + TryFrom<Resource> + 'static,
{
	/// Start up a new Unpaged<R> stream.
	pub fn from_searchset(client: Client<FhirStu3>, searchset: Bundle) -> Self {
		let page = Page::from_searchset(client.clone(), searchset);

		Self { client, page, future_next_page: None }
	}
}

impl<R> Stream for Unpaged<R>
where
	R: NamedResource + DomainResource + TryFrom<Resource> + 'static,
{
	type Item = Result<R, Error>;

	fn poll_next(
		mut self: Pin<&mut Self>,
		cx: &mut std::task::Context<'_>,
	) -> Poll<Option<Self::Item>> {
		let span = tracing::trace_span!("Unpaged::poll_next");
		let _span_guard = span.enter();

		// If there are still matches left, get the next one
		if !self.page.is_empty() {
			tracing::trace!("Unpaged::page is not empty, polling for next match");
			if let Poll::Ready(res) = self.page.poll_next_unpin(cx) {
				if let Some(r) = res {
					tracing::debug!("Next match in Unpaged::matches available");
					return Poll::Ready(Some(r));
				} else {
					tracing::debug!("Unpaged::matches is empty, waiting for next page");
				}
			}
		}

		if let Some(future_next_page) = self.future_next_page.as_mut() {
			tracing::trace!("Unpaged::future_next_page is set, polling for next page");
			if let Poll::Ready(next_page) = future_next_page.as_mut().poll(cx) {
				self.future_next_page = None;
				tracing::debug!("Next page fetched and ready");

				self.page = match next_page {
					Ok(page) => page,
					Err(e) => {
						tracing::error!("Fetching next page returned error: {}", e);

						return Poll::Ready(Some(Err(e)));
					}
				};
			}
		// Start retrieving the next page if we have a next URL and there is no next page being fetched.
		} else if let Some(next_page) = self.page.next_page() {
			tracing::debug!("Current page has next URL, starting to fetch next page");
			self.future_next_page = Some(next_page.boxed());
		}

		// Else check if all resources were consumed or if we are waiting for new
		// resources to arrive.
		if !self.page.is_empty() {
			tracing::trace!("Unpaged results waiting for remaining resources in current page");
			cx.waker().wake_by_ref();
			Poll::Pending
		} else if self.future_next_page.is_some() {
			tracing::trace!("Unpaged results waiting for response to next URL fetch");
			cx.waker().wake_by_ref();
			Poll::Pending
		} else {
			tracing::debug!("Unpaged results exhausted");
			Poll::Ready(None)
		}
	}

	fn size_hint(&self) -> (usize, Option<usize>) {
		let (page_min, page_max) = self.page.size_hint();

		if self.page.next_page().is_some() {
			(page_min, None)
		} else {
			(page_min, page_max)
		}
	}
}

async fn fetch_resource<R: TryFrom<Resource>>(
	client: Client<FhirStu3>,
	url: Url,
) -> Result<R, Error> {
	let url_str = url.to_string();

	// Make sure we are not forwarded to any malicious server.
	if url.origin() != client.0.base_url.origin() {
		return Err(Error::DifferentOrigin(url_str));
	}

	// Fetch a single resource from the given URL.
	let resource = client.read_generic(url).await?;
	resource.ok_or_else(|| Error::ResourceNotFound(url_str))
}

impl<R> std::fmt::Debug for Unpaged<R> {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		f.debug_struct("Unpaged")
			.field("client", &self.client)
			.field("page", &self.page)
			.field("future_next_page", &self.future_next_page.as_ref().map(|_| "_"))
			.finish()
	}
}

/// Stream that yields each match entry in a searchset Bundle. Tries to fetch entries from the fullUrl property
/// if the resource field is empty. Fills resource reference target fields with the referred to resources,
/// if available in the Bundle.
pub struct Page<R> {
	client: Client<FhirStu3>,
	bundle: Bundle,
	matches: VecDeque<BundleEntry>,
	future_resource: Option<BoxFuture<'static, Result<R, Error>>>,
}

impl<R> Page<R>
where
	R: NamedResource + DomainResource + TryFrom<Resource> + 'static,
{
	pub fn from_searchset(client: Client<FhirStu3>, bundle: Bundle) -> Page<R> {
		assert!(
			bundle.r#type == BundleType::Searchset,
			"unable to get search matches from non-searchset Bundles"
		);

		// I hate cloning here but I didn't want to have to add lifetime params all over the place.
		// We shall see if it affects performance.
		let matches: VecDeque<_> = bundle
			.entry
			.iter()
			.flatten()
			.filter(|e| {
				e.search
					.as_ref()
					.and_then(|s| s.mode.as_ref())
					.map_or(true, |m| *m == SearchEntryMode::Match)
			})
			.cloned()
			.collect();

		Self { client, bundle, matches, future_resource: None }
	}

	fn is_empty(&self) -> bool {
		self.matches.is_empty()
	}
}

impl<R> Paged<R> for Page<R>
where
	R: NamedResource + DomainResource + TryFrom<Resource> + 'static,
{
	fn next_page(&self) -> Option<impl Future<Output = Result<Self, Error>> + 'static> {
		let next_url = find_next_page_url(&self.bundle)?;
		let client = self.client.clone();

		let fut = async move {
			let searchset: Bundle = client.fetch_resource(next_url?).await?;

			Ok(Page::from_searchset(client, searchset))
		};

		Some(fut.boxed())
	}
}

impl<R> Stream for Page<R>
where
	R: NamedResource + DomainResource + TryFrom<Resource> + 'static,
{
	type Item = Result<R, Error>;

	fn poll_next(
		mut self: Pin<&mut Self>,
		cx: &mut std::task::Context<'_>,
	) -> Poll<Option<Self::Item>> {
		let span = tracing::trace_span!("SearchMatches::poll_next");
		let _span_guard = span.enter();

		// Check on resource fetch future first to output as the next resource.
		if let Some(future_resource) = self.future_resource.as_mut() {
			return match ready!(future_resource.as_mut().poll(cx)) {
				Ok(mut resource) => {
					tracing::trace!("Next `fullUrl` fetched resource ready");

					self.future_resource = None;
					populate_reference_targets_internal(
						&self.client.0.base_url,
						&mut resource,
						Some(&self.bundle),
						None,
					);

					Poll::Ready(Some(Ok(resource)))
				}
				Err(e) => Poll::Ready(Some(Err(e))),
			};
		}

		// Otherwise get the next match from the list
		while let Some(entry) = self.matches.pop_front() {
			if let Some(resource) = entry.resource {
				tracing::debug!("Found next Bundle entry to return");
				let resource_type = resource.resource_type();

				let mut r: R = resource.try_into().map_err(|_| {
					Error::WrongResourceType(resource_type.to_string(), R::TYPE.to_string())
				})?;

				populate_reference_targets_internal(
					&self.client.0.base_url,
					&mut r,
					Some(&self.bundle),
					None,
				);

				return Poll::Ready(Some(Ok(r)));
			} else if let Some(url) = entry.full_url {
				if let Ok(url) = Url::parse(&url) {
					tracing::debug!("Next entry needs to be fetched, starting to fetch it");

					self.future_resource = Some(fetch_resource(self.client.clone(), url).boxed());
					cx.waker().wake_by_ref();

					return Poll::Pending;
				} else {
					tracing::error!("Could not parse next entry URL");

					return Poll::Ready(Some(Err(Error::UrlParse(url))));
				}
			}
		}

		tracing::trace!("SearchMatches exhausted");
		Poll::Ready(None)
	}

	fn size_hint(&self) -> (usize, Option<usize>) {
		let size = self.matches.len();

		(size, Some(size))
	}
}

impl<R> std::fmt::Debug for Page<R> {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		f.debug_struct("Page")
			.field("client", &self.client)
			.field("bundle", &self.bundle)
			.field("matches", &self.matches)
			.field("future_resource", &self.future_resource.as_ref().map(|_| "_"))
			.finish()
	}
}
