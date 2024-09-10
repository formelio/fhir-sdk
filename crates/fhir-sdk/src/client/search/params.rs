//! Search parameters

use std::hash::{Hash, Hasher};
use std::marker::PhantomData;

/// A collection of AND-joined search parameters.
#[derive(Debug)]
pub struct SearchParameters<R> {
	/// List of search queries.
	queries: Vec<(String, String)>,

	resource_type: PhantomData<R>,
}

impl<R> Clone for SearchParameters<R> {
	fn clone(&self) -> Self {
		Self { queries: self.queries.clone(), resource_type: PhantomData::default() }
	}
}

impl<R> Hash for SearchParameters<R> {
	fn hash<H: Hasher>(&self, state: &mut H) {
		self.queries.hash(state);
	}
}

impl<R> PartialEq for SearchParameters<R> {
	fn eq(&self, other: &Self) -> bool {
		self.queries.eq(&other.queries)
	}
}

impl<R> Eq for SearchParameters<R> {}

impl<R> Default for SearchParameters<R> {
	fn default() -> Self {
		Self::empty()
	}
}

impl<R> SearchParameters<R> {
	/// Create a new list of [`SearchParameters`].
	#[must_use]
	pub fn empty() -> Self {
		Self { queries: Vec::new(), resource_type: PhantomData }
	}

	/// Initialize a new [`SearchParameters`] with a parameter
	pub fn with<P>(field: &str, parameter: P) -> Self
	where
		P: SearchParameter,
	{
		Self::empty().and(field, parameter)
	}

	/// Add a search parameter.
	#[must_use]
	pub fn and<P>(mut self, key: &str, parameter: P) -> Self
	where
		P: SearchParameter,
	{
		self.add(key, parameter);
		self
	}

	/// Add other types of parameters to the query string
	pub fn include<Q>(mut self, query: Q) -> Self
	where
		Q: IntoQuery,
	{
		self.add_query(query);
		self
	}

	/// Add a raw custom search parameter.
	///
	/// The key of the search query includes modifiers or chaining.
	///
	/// The value of the search query might include multiple comma-separated
	/// values. A value can consist of pipe-separated values for token search or
	/// can be prepended by a comparator like `lt` for numbers.
	#[must_use]
	pub fn and_raw(mut self, key: impl Into<String>, value: impl ToString) -> Self {
		self.add_raw(key, value);
		self
	}

	/// Convert to a list of raw queries.
	pub(crate) fn into_queries(self) -> Vec<(String, String)> {
		self.queries
	}

	/// Add other types of parameters to the query string without consuming `self`
	pub fn add<P>(&mut self, key: &str, parameter: P)
	where
		P: SearchParameter,
	{
		self.add_query((key, parameter))
	}

	/// Add a query parameter without consuming `self`
	pub fn add_query<Q>(&mut self, query: Q)
	where
		Q: IntoQuery,
	{
		let (key, value) = query.into_query();

		self.add_raw(key, value);
	}

	pub(super) fn add_all(&mut self, parameters: SearchParameters<R>) {
		for (key, value) in parameters.into_queries() {
			self.add_raw(key, value);
		}
	}

	pub(super) fn add_raw(&mut self, key: impl Into<String>, value: impl ToString) {
		self.queries.push((key.into(), value.to_string()));
	}
}

impl<Q, R> FromIterator<Q> for SearchParameters<R>
where
	Q: IntoQuery,
{
	fn from_iter<T: IntoIterator<Item = Q>>(iter: T) -> Self {
		iter.into_iter().fold(SearchParameters::empty(), |params, param| params.include(param))
	}
}

/// Functionality to convert a search parameter to the URL query.
pub trait IntoQuery {
	/// Convert this search parameter into the query pair (key, value).
	fn into_query(self) -> (String, String);
}

impl<S: SearchParameter> IntoQuery for (&str, S) {
	fn into_query(self) -> (String, String) {
		let (key, param) = self;

		let key = if let Some(modifier) = param.modifier() {
			format!("{key}:{modifier}")
		} else {
			key.to_string()
		};

		(key, param.query_value())
	}
}

/// A FHIR search parameter
pub trait SearchParameter: Sized {
	/// URL safe query string value of the parameter
	fn query_value(&self) -> String;

	/// Modifier of the parameter, like :in, :not-in. Without the :
	fn modifier(&self) -> Option<&str> {
		None
	}

	/// Or this parameter with another parameter
	fn or(self, param: Self) -> SearchParameterOrList<Self> {
		SearchParameterOrList::new(self).or(param)
	}
}

/// A set of OR-ed FHIR search parameters
#[derive(Clone, Debug)]
pub struct SearchParameterOrList<P> {
	// Used to validate that every added parameter has the same modifier
	modifier: Option<String>,

	// The parameters that should be OR'ed
	params: Vec<P>,
}

impl<P: SearchParameter> SearchParameterOrList<P> {
	/// Create an empty [SearchParameterOrList]
	pub fn empty() -> Self {
		Self { modifier: None, params: Vec::new() }
	}

	/// Create a new [SearchParameterOrList]
	pub fn new(param: P) -> Self {
		Self { modifier: param.modifier().map(|m| m.to_string()), params: vec![param] }
	}

	/// Add an OR search parameter
	pub fn or(mut self, param: P) -> Self {
		if self.modifier.as_deref() != param.modifier() {
			panic!("cannot OR two search parameters with different modifiers");
		}

		self.params.push(param);
		self
	}
}

impl<P: SearchParameter> SearchParameter for SearchParameterOrList<P> {
	fn modifier(&self) -> Option<&str> {
		self.modifier.as_deref()
	}

	fn query_value(&self) -> String {
		self.params.iter().map(|p| p.query_value()).collect::<Vec<String>>().join(",")
	}
}

impl<P> FromIterator<P> for SearchParameterOrList<P>
where
	P: SearchParameter,
{
	fn from_iter<T: IntoIterator<Item = P>>(iter: T) -> Self {
		iter.into_iter().fold(SearchParameterOrList::empty(), |ors, param| ors.or(param))
	}
}

/// Escape a search parameter value.
pub(crate) fn escape_value(value: &str) -> String {
	value.replace('\\', "\\\\").replace('|', "\\|").replace('$', "\\$").replace(',', "\\,")
}

#[cfg(test)]
mod tests {
	use super::*;

	struct MyParam<'a> {
		modifier: Option<&'a str>,
		value: &'a str,
	}

	impl SearchParameter for MyParam<'_> {
		fn modifier(&self) -> Option<&str> {
			self.modifier
		}

		fn query_value(&self) -> String {
			self.value.to_string()
		}
	}

	#[test]
	fn or() {
		let params =
			MyParam { modifier: None, value: "bla" }.or(MyParam { modifier: None, value: "ble" });

		assert_eq!(params.query_value(), "bla,ble");
	}

	#[test]
	#[should_panic]
	fn or_different_modifiers() {
		MyParam { modifier: None, value: "bla" }
			.or(MyParam { modifier: Some("mod"), value: "ble" });
	}
}
