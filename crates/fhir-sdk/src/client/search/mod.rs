//! Search handling.

mod params;

pub use params::*;

use std::hash::Hash;
use std::marker::PhantomData;

use async_trait::async_trait;
use futures::{Future, Stream};
use reqwest::Url;

use super::{Client, Error};

/// A FHIR search that automatically resolves next pages
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct UnpagedSearch<E, R> {
	/// The executor of the search (e.g. the [Client])
	executor: Option<E>,

	/// Search parameters.
	params: SearchParameters<R>,
}

impl<E, R> UnpagedSearch<E, R>
where
	E: UnpagedSearchExecutor<R>,
{
	/// Creates a new [UnpagedSearch] without an executor. An executor needs to be set with [UnpagedSearch::with_executor] before calling [UnpagedSearch::send].
	/// If [UnpagedSearch::send] is called without an executor being set, it will panic.
	///
	/// An [UnpagedSearch] is useful if you're looking for a way to represent a search that can be executed on an arbitrary or currently unknown [Client].
	/// If you just want to have your [Client] do a search, you are probably looking for [Client::search] instead.
	pub fn new() -> Self {
		Self { executor: None, params: SearchParameters::empty() }
	}

	/// Sets a search executor (i.e. a [Client])
	pub fn with_executor(mut self, executor: E) -> Self {
		self.executor = Some(executor);
		self
	}

	/// Add a search parameter
	pub fn with<P>(mut self, field: &str, parameter: P) -> Self
	where
		P: SearchParameter,
	{
		self.params.add(field, parameter);
		self
	}

	/// Add an existing set of search parameters
	pub fn with_params(mut self, parameters: SearchParameters<R>) -> Self {
		self.params.add_all(parameters);
		self
	}

	/// Add a search parameter as a raw string.
	///
	/// Prefer [Search::with] if possible.
	pub fn with_raw(mut self, key: impl Into<String>, value: impl ToString) -> Self {
		self.params.add_raw(key, value);
		self
	}

	/// Add a search parameter. Alias of [Search::with].
	pub fn and<P>(self, field: &str, parameter: P) -> Self
	where
		P: SearchParameter,
	{
		self.with(field, parameter)
	}

	/// Include other query parameters
	pub fn include<Q>(mut self, query: Q) -> Self
	where
		Q: IntoQuery,
	{
		self.params.add_query(query);
		self
	}

	/// Add a search parameter as a string. Alias of [Search::with_raw].
	///
	/// Prefer [Search::and] if possible.
	pub fn and_raw(self, key: impl Into<String>, value: impl ToString) -> Self {
		self.with_raw(key, value)
	}

	/// Execute the search
	pub async fn send(self) -> Result<E::Stream, Error> {
		self.executor.expect("no search executor set").search_unpaged(self.params).await
	}
}

impl<E, R> UnpagedSearch<E, R>
where
	E: UnpagedSearchExecutor<R> + PagedSearchExecutor<R>,
{
	/// Transform this search into a paged search
	pub fn paged(self, page_size: Option<u32>) -> PagedSearch<E, R> {
		let Self { executor, params } = self;

		PagedSearch { executor, params, page_size }
	}
}

#[derive(Debug)]
pub struct PagedSearch<E, R> {
	/// The executor of the search (e.g. the [Client])
	executor: Option<E>,

	/// Search parameters.
	params: SearchParameters<R>,

	/// Preferred page size
	page_size: Option<u32>,
}

impl<E, R> PagedSearch<E, R>
where
	E: PagedSearchExecutor<R>,
{
	/// Execute the search
	pub async fn send(self) -> Result<(E::Stream, Option<NextPageCursor<E, R>>), Error> {
		self.executor
			.expect("no search executor set")
			.search_paged(self.params, self.page_size)
			.await
	}
}

/// Stream of resources returned by [Search::send].
pub trait Paged<R>: Stream<Item = Result<R, Error>> + Send + Sized {
	/// If the search is paged, returns the next page. Returns `None` if the [SearchResponse] is
	/// unpaged or if there is no next page available.
	fn next_page(&self) -> Option<impl Future<Output = Result<Self, Error>> + 'static>;
}

#[derive(Debug)]
pub struct NextPageCursor<E, R> {
	executor: E,
	next_page_url: Url,
	resource_type: PhantomData<R>,
}

impl<E, R> NextPageCursor<E, R>
where
	E: PagedSearchExecutor<R> + 'static,
{
	pub fn new(executor: E, next_page_url: Url) -> Self {
		Self { executor, next_page_url, resource_type: PhantomData }
	}

	pub async fn next_page(self) -> Result<(E::Stream, Option<Self>), Error> {
		self.executor.fetch_next_page(self.next_page_url).await
	}
}

/// Executor of unpaged FHIR searches (e.g. [Client])
#[async_trait]
pub trait UnpagedSearchExecutor<R>: Sized {
	/// The stream of FHIR resources that will be returned
	type Stream: Stream<Item = Result<R, Error>>;

	/// Execute an unpaged search
	async fn search_unpaged(self, params: SearchParameters<R>) -> Result<Self::Stream, Error>;
}

/// Executor of paged FHIR searches (e.g. [Client])
#[async_trait]
pub trait PagedSearchExecutor<R>: Sized {
	/// The stream of FHIR resources that will be returned
	type Stream: Stream<Item = Result<R, Error>>;

	async fn search_paged(
		self,
		params: SearchParameters<R>,
		page_size: Option<u32>,
	) -> Result<(Self::Stream, Option<NextPageCursor<Self, R>>), Error>;

	async fn fetch_next_page(
		self,
		next_page_url: Url,
	) -> Result<(Self::Stream, Option<NextPageCursor<Self, R>>), Error>;
}

impl<V: 'static> Client<V> {
	/// Start constructing a search for FHIR resources of a given type.
	/// Only returns matches. Populates reference target fields with any
	/// matching included resources.
	pub fn search<R>(&self) -> UnpagedSearch<Self, R>
	where
		Self: UnpagedSearchExecutor<R>,
		R: Send,
	{
		UnpagedSearch::new().with_executor(self.clone())
	}
}
