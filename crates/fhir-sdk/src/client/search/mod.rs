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
#[derive(Debug, Clone)]
pub struct Search<E, R> {
	/// The executor of the search (e.g. the [Client])
	executor: Option<E>,

	/// Search parameters.
	params: SearchParameters<R>,
}

impl<E, R> Search<E, R> {
	/// Creates a new [Search] without an executor. An executor needs to be set with [Search::with_executor] before calling [Search::send].
	/// If [Search::send] is called without an executor being set, it will panic.
	///
	/// An [Search] is useful if you're looking for a way to represent a search that can be executed on an arbitrary or currently unknown [Client].
	/// If you just want to have your [Client] do a search, you are probably looking for [Client::search] instead.
	pub fn new() -> Self {
		Self { executor: None, params: SearchParameters::empty() }
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

	/// Add other types of parameters to the query string without consuming `self`
	pub fn add<P>(&mut self, key: &str, parameter: P)
	where
		P: SearchParameter,
	{
		self.params.add_query((key, parameter))
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

	/// Add a query parameter without consuming `self`
	pub fn add_query<Q>(&mut self, query: Q)
	where
		Q: IntoQuery,
	{
		let (key, value) = query.into_query();

		self.params.add_raw(key, value);
	}

	/// Add a search parameter as a string. Alias of [Search::with_raw].
	///
	/// Prefer [Search::and] if possible.
	pub fn and_raw(self, key: impl Into<String>, value: impl ToString) -> Self {
		self.with_raw(key, value)
	}
}

#[async_trait]
impl<E, R> ExecutableSearch<E, R> for Search<E, R>
where
	E: SearchExecutor<R> + Send,
	E::Stream: Send,
	R: Send,
{
	type Value = E::Stream;

	fn with_executor(mut self, executor: E) -> Self {
		self.executor = Some(executor);
		self
	}

	async fn send(self) -> Result<Self::Value, Error> {
		self.executor.expect("no search executor set").search(self.params).await
	}
}

impl<E, R> Hash for Search<E, R>
where
	R: Hash,
{
	fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
		self.params.hash(state)
	}
}

impl<E, R> PartialEq for Search<E, R> {
	fn eq(&self, other: &Self) -> bool {
		self.params.eq(&other.params)
	}
}

impl<E, R> Eq for Search<E, R> {}

impl<E, R> Search<E, R> {
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
	pub async fn send(self) -> Result<(E::Stream, Option<NextPageCursor<E, R>>), Error> {
		self.executor
			.expect("no search executor set")
			.search_paged(self.params, self.page_size)
			.await
	}
}

impl<E, R> Hash for PagedSearch<E, R>
where
	R: Hash,
{
	fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
		self.params.hash(state);
		self.page_size.hash(state);
	}
}

impl<E, R> PartialEq for PagedSearch<E, R> {
	fn eq(&self, other: &Self) -> bool {
		self.params.eq(&other.params)
	}
}

impl<E, R> Eq for PagedSearch<E, R> {}

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

/// Trait implemented by the different search types, allowing them to
/// define their own return value.
#[async_trait]
pub trait ExecutableSearch<E, R> {
	/// The type to return
	type Value: Stream<Item = Result<R, Error>>;

	/// Set a search executor for this search
	fn with_executor(self, executor: E) -> Self;

	/// Send out the search request
	async fn send(self) -> Result<Self::Value, Error>;
}

/// Executor of unpaged FHIR searches (e.g. [Client])
#[async_trait]
pub trait SearchExecutor<R>: Sized {
	/// The stream of FHIR resources that will be returned
	type Stream: Stream<Item = Result<R, Error>>;

	/// Execute an unpaged search
	async fn search(self, params: SearchParameters<R>) -> Result<Self::Stream, Error>;
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

impl<V: 'static + Send> Client<V> {
	/// Start constructing a search for FHIR resources of a given type.
	/// Only returns matches. Populates reference target fields with any
	/// matching included resources.
	pub fn search<R>(&self) -> Search<Self, R>
	where
		Self: SearchExecutor<R>,
		<Self as SearchExecutor<R>>::Stream: Send,
		R: Send,
	{
		Search::new().with_executor(self.clone())
	}
}
