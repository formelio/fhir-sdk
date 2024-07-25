use super::SearchParamType;

#[derive(Debug)]
pub struct SearchParam {
	/// Name for this search parameter (computer friendly)
	pub name: String,

	/// Natural language description of the search parameter
	pub description: String,

	/// Recommended name for parameter in search url
	pub code: String,

	/// Base types this search parameter applies to
	pub base: Vec<String>,

	/// The type of value that a search parameter may contain, and how the content is interpreted
	pub r#type: SearchParamType,

	/// FHIRPath expression resolving to all the fields this search parameter points at
	pub expression: String,

	/// Comparators supported for the search parameter.
	pub comparators: Vec<SearchComparator>,
}

#[derive(Debug, PartialEq, Eq, Clone, Hash)]
pub enum SearchComparator {
	Eq,
	Ne,
	Gt,
	Lt,
	Ge,
	Le,
	Sa,
	Eb,
	Ap,
}
