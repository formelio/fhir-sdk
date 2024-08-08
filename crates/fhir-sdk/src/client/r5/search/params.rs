use std::str::FromStr;

use crate::client::search::{escape_value, IntoQuery, SearchParameter};
use fhir_model::r5::{
	codes::{SearchComparator, SearchModifierCode},
	resources::ResourceType,
};
use fhir_model::ParsedReference;
use thiserror::Error;

/// Number search.
///
/// Only implements most common functionality. Refer to adding raw queries when
/// this does not suffice.
#[derive(Debug, Clone)]
pub struct NumberParam<'a> {
	/// How to compare the value
	pub comparator: Option<SearchComparator>,
	/// The numeric value
	pub value: &'a str,
}

impl<'a> NumberParam<'a> {
	/// Create a new `[NumberParam]`
	pub fn new(value: &'a str) -> Self {
		Self { comparator: None, value }
	}

	/// Set a comparator
	pub fn comparator(mut self, comparator: SearchComparator) -> Self {
		self.comparator = Some(comparator);
		self
	}
}

impl<'a> SearchParameter for NumberParam<'a> {
	fn query_value(&self) -> String {
		let value = escape_value(self.value);

		if let Some(comparator) = self.comparator {
			format!("{}{}", comparator.as_ref(), value)
		} else {
			value
		}
	}
}

/// Date search.
///
/// Only implements most common functionality. Refer to adding raw queries when
/// this does not suffice.
#[derive(Debug, Clone, Copy)]
pub struct DateParam<'a> {
	/// Search comparator to compare the date.
	pub comparator: Option<SearchComparator>,
	/// Value to search for.
	pub value: &'a str,
}

impl<'a> DateParam<'a> {
	/// Create a new `[DateParam]`
	pub fn new(value: &'a str) -> Self {
		Self { comparator: None, value }
	}

	/// Set a comparator
	pub fn comparator(mut self, comparator: SearchComparator) -> Self {
		self.comparator = Some(comparator);
		self
	}
}

impl<'a> SearchParameter for DateParam<'a> {
	fn query_value(&self) -> String {
		let value = escape_value(self.value);

		if let Some(comparator) = self.comparator {
			format!("{}{}", comparator.as_ref(), value)
		} else {
			value
		}
	}
}

/// String search.
///
/// Only implements most common functionality. Refer to adding raw queries when
/// this does not suffice.
#[derive(Debug, Clone, Copy)]
pub enum StringParam<'a> {
	/// Standard string search. This is a case-insensitive starts-with search.
	Standard(&'a str),
	/// Search a string that contains the given value.
	Contains(&'a str),
	/// Search a string that matches exactly the value, including casing and
	/// accents.
	Exact(&'a str),
}

impl<'a> SearchParameter for StringParam<'a> {
	fn modifier(&self) -> Option<&str> {
		match self {
			Self::Contains(_) => Some(SearchModifierCode::Contains.as_ref()),
			Self::Exact(_) => Some(SearchModifierCode::Exact.as_ref()),
			_ => None,
		}
	}

	fn query_value(&self) -> String {
		match self {
			Self::Standard(v) => v.to_string(),
			Self::Contains(v) => v.to_string(),
			Self::Exact(v) => v.to_string(),
		}
	}
}

/// Token search, e.g. in `CodeableConcept`s or `identifier`s.
///
/// Only implements most common functionality. Refer to adding raw queries when
/// this does not suffice.
#[derive(Debug, Clone, Copy)]
pub enum TokenParam<'a> {
	/// Match a specific code in a specific system
	CodeInSystem {
		/// The system
		system: &'a str,
		/// The code
		code: &'a str,
		/// Whether to negate the search
		not: bool,
	},
	/// Match a specific code in any system
	CodeInAnySystem {
		/// The code
		code: &'a str,
		/// Whether to negate the match
		not: bool,
	},
	/// Match a code that does not have a system
	CodeWithoutSystem {
		/// The code
		code: &'a str,
		/// Whether to negate the search
		not: bool,
	},
	/// Match any code in a specific system
	InSystem {
		/// The system
		system: &'a str,
		/// Whether to negate the search
		not: bool,
	},
	/// Token search with the `of-type` modifier. Only possible on
	/// `identifier`s.
	OfType {
		/// System of type to search on.
		type_system: Option<&'a str>,
		/// Code of the type to search on.
		type_code: Option<&'a str>,
		/// Value to search on with the given type.
		value: Option<&'a str>,
	},
	/// Token search whether the value is `in` or `not-in` a given `ValueSet`.
	In {
		/// `ValueSet` reference URI.
		value_set: &'a str,
		/// Whether to switch to `not-in` search.
		not: bool,
	},
	/// Tests the `text` or `display` values.
	CodeText {
		/// Text to search for (is a starts-with search).
		text: &'a str,
	},
}

impl<'a> TokenParam<'a> {
	/// Token param for a code within a specific system
	pub const fn code_in_system(system: &'a str, code: &'a str) -> Self {
		Self::CodeInSystem { system, code, not: false }
	}

	/// Token param for any code within the given system
	pub const fn in_system(system: &'a str) -> Self {
		Self::InSystem { system, not: false }
	}

	/// Token param for a specific code without specifying system
	pub const fn code(code: &'a str) -> Self {
		Self::CodeInAnySystem { code, not: false }
	}

	/// Match a specific code that does not have a system
	pub const fn code_without_system(code: &'a str) -> Self {
		Self::CodeWithoutSystem { code, not: false }
	}
}

impl<'a> SearchParameter for TokenParam<'a> {
	fn modifier(&self) -> Option<&str> {
		match self {
			TokenParam::CodeInSystem { not, .. }
			| TokenParam::CodeWithoutSystem { not, .. }
			| TokenParam::CodeInAnySystem { not, .. }
			| TokenParam::CodeInSystem { not, .. }
				if *not =>
			{
				Some(SearchModifierCode::Not.as_ref())
			}
			TokenParam::OfType { .. } => Some(SearchModifierCode::OfType.as_ref()),
			TokenParam::In { not, .. } if *not => Some(SearchModifierCode::NotIn.as_ref()),
			TokenParam::In { .. } => Some(SearchModifierCode::In.as_ref()),
			TokenParam::CodeText { .. } => Some(SearchModifierCode::CodeText.as_ref()),
			_ => None,
		}
	}

	fn query_value(&self) -> String {
		match self {
			TokenParam::CodeInSystem { system, code, .. } => format!("{system}|{code}"),
			TokenParam::CodeInAnySystem { code, .. } => format!("{code}"),
			TokenParam::CodeWithoutSystem { code, .. } => format!("|{code}"),
			TokenParam::InSystem { system, .. } => format!("{system}|"),
			TokenParam::OfType { type_system, type_code, value } => format!(
				"{}|{}|{}",
				escape_value(type_system.unwrap_or_default()),
				escape_value(type_code.unwrap_or_default()),
				escape_value(value.unwrap_or_default())
			),
			TokenParam::In { value_set, .. } => escape_value(value_set),
			TokenParam::CodeText { text } => escape_value(text),
		}
	}
}

/// Search in references. Includes chaining, i.e. querying fields of the target
/// resource.
///
/// Only implements most common functionality. Refer to adding raw queries when
/// this does not suffice.
#[derive(Debug, Clone, Copy)]
pub enum ReferenceParam<'a> {
	/// Standard reference search by relative reference.
	Standard {
		/// Resource type of the resource.
		resource_type: ResourceType,
		/// ID of the resource the reference should point to.
		id: &'a str,
		/// Historic version id to search for.
		version_id: Option<&'a str>,
	},
	/// Standard reference search by absolute URL.
	Url {
		/// The FHIR base URL for FHIR server references, or the whole
		/// URL for non-FHIR server URLs or URNs
		base_url: &'a str,
		/// Resource type
		resource_type: Option<&'a str>,
		/// Resource ID
		id: Option<&'a str>,
		/// Specific version id to search for.
		version_id: Option<&'a str>,
	},
	/// Reference search by the `.identifier` field.
	Identifier(TokenParam<'a>),
}

impl<'a> ReferenceParam<'a> {
	/// Reference to the resource with the given ID
	pub fn new(resource_type: ResourceType, id: &'a str) -> Self {
		Self::versioned(resource_type, id, None)
	}

	/// Create a new versioned reference parameter
	pub fn versioned(
		resource_type: ResourceType,
		id: &'a str,
		version_id: Option<&'a str>,
	) -> Self {
		Self::Standard { resource_type, id, version_id }
	}
}

/// Local references (to contained resources) cannot be used as search parameters
#[derive(Debug, Error)]
#[error("local references cannot be used as FHIR search parameters")]
pub struct LocalReferenceUnusableAsParameter;

impl<'a> TryFrom<ParsedReference<'a>> for ReferenceParam<'a> {
	type Error = LocalReferenceUnusableAsParameter;

	fn try_from(reference: ParsedReference<'a>) -> Result<Self, Self::Error> {
		match reference {
			ParsedReference::Local { .. } => Err(LocalReferenceUnusableAsParameter),
			ParsedReference::Relative { resource_type, id, version_id } => {
				Ok(ReferenceParam::versioned(
					ResourceType::from_str(resource_type).unwrap(),
					id,
					version_id,
				))
			}
			ParsedReference::Absolute { base_url, resource_type, id, version_id } => {
				Ok(ReferenceParam::Url { base_url, resource_type, id, version_id })
			}
		}
	}
}

impl<'a> SearchParameter for ReferenceParam<'a> {
	fn modifier(&self) -> Option<&str> {
		match self {
			ReferenceParam::Identifier { .. } => Some(SearchModifierCode::Identifier.as_ref()),
			_ => None,
		}
	}

	fn query_value(&self) -> String {
		match self {
			Self::Standard { resource_type, id, version_id: Some(version_id) } => {
				escape_value(&format!("{resource_type}/{id}/_history/{version_id}"))
			}
			Self::Standard { resource_type, id, version_id: None } => {
				escape_value(&format!("{resource_type}/{id}"))
			}
			Self::Url { base_url, resource_type: None, id: None, version_id: None } => {
				escape_value(base_url)
			}
			Self::Url {
				base_url,
				resource_type: Some(resource_type),
				id: Some(id),
				version_id: None,
			} => escape_value(&format!("{base_url}/{resource_type}/{id}")),
			Self::Url {
				base_url,
				resource_type: Some(resource_type),
				id: Some(id),
				version_id: Some(version_id),
			} => escape_value(&format!("{base_url}/{resource_type}/{id}/_history/{version_id}")),
			Self::Url { base_url, resource_type: None, id: None, version_id: Some(version_id) } => {
				format!("{}|{}", escape_value(base_url), escape_value(version_id))
			}
			Self::Identifier(token) => token.query_value(),
			_ => panic!("Invalid reference parameter"),
		}
	}
}

/// Search on a quantity.
///
/// Only implements most common functionality. Refer to adding raw queries when
/// this does not suffice.
#[derive(Debug, Clone, Copy)]
pub struct QuantityParam<'a> {
	/// Search comparator to compare the date.
	pub comparator: Option<SearchComparator>,
	/// Value to search for.
	pub value: &'a str,
	/// Optional system.
	pub system: Option<&'a str>,
	/// Optional code.
	pub code: Option<&'a str>,
}

impl<'a> QuantityParam<'a> {
	/// Quantity with a given value
	pub fn new(value: &'a str) -> Self {
		Self { comparator: None, value, system: None, code: None }
	}

	/// Quantity with a given value
	pub fn with_comparator(mut self, comparator: SearchComparator) -> Self {
		self.comparator = Some(comparator);
		self
	}
}

impl<'a> SearchParameter for QuantityParam<'a> {
	fn query_value(&self) -> String {
		let value = if let Some(comparator) = self.comparator {
			format!("{}{}", comparator.as_ref(), escape_value(self.value))
		} else {
			escape_value(self.value)
		};

		if self.system.is_some() || self.code.is_some() {
			format!(
				"{value}|{}|{}",
				escape_value(self.system.unwrap_or_default()),
				escape_value(self.code.unwrap_or_default())
			)
		} else {
			value
		}
	}
}

/// Search on a URI.
///
/// Only implements most common functionality. Refer to adding raw queries when
/// this does not suffice.
#[derive(Debug, Clone, Copy)]
pub enum UriParam<'a> {
	/// Standard URI search, that matches exactly.
	Standard(&'a str),
	/// Match any URL that is below the given URL path, so contains more URL
	/// segments.
	Below(&'a str),
	/// Match any URL that is above the given URL path, so contains less URL
	/// segments.
	Above(&'a str),
}

impl<'a> UriParam<'a> {
	/// Create a new URI search parameter
	pub fn new(value: &'a str) -> Self {
		Self::Standard(value)
	}
}

impl<'a> SearchParameter for UriParam<'a> {
	fn modifier(&self) -> Option<&str> {
		match self {
			UriParam::Below(_) => Some(SearchModifierCode::Below.as_ref()),
			UriParam::Above(_) => Some(SearchModifierCode::Above.as_ref()),
			_ => None,
		}
	}

	fn query_value(&self) -> String {
		let uri = match self {
			Self::Standard(u) => u,
			Self::Below(u) => u,
			Self::Above(u) => u,
		};

		escape_value(uri)
	}
}

/// Search on any item whether it is a missing field using the `missing`
/// modifier.
#[derive(Debug, Clone, Copy)]
pub struct MissingParam(bool);

impl SearchParameter for MissingParam {
	fn modifier(&self) -> Option<&str> {
		Some(SearchModifierCode::Missing.as_ref())
	}

	fn query_value(&self) -> String {
		self.0.to_string()
	}
}

/// Include referred to resources in the search response
#[derive(Debug, Clone, Copy)]
pub struct IncludeParam<'a> {
	/// Resource type from which the join comes
	pub source_type: ResourceType,

	/// Field name to join on. Must be a search parameter of type reference for the [IncludeParam::source] resource type.
	pub field: &'a str,

	/// Type of the target resource in the case the reference field can have multiple target resource types.
	pub target_type: Option<ResourceType>,

	/// Whether to recursively include.
	pub iterate: bool,

	/// Whether this is a reverse include.
	pub reverse: bool,
}

impl<'a> IncludeParam<'a> {
	/// Create a new `_include` parameter
	pub fn new(source_type: ResourceType, field: &'a str) -> Self {
		Self { source_type, field, target_type: None, iterate: false, reverse: false }
	}

	/// Create a new `_revInclude` parameter
	pub fn reverse(source_type: ResourceType, field: &'a str) -> Self {
		Self { source_type, field, target_type: None, iterate: false, reverse: true }
	}

	/// Add the `:iterate` modifier
	pub fn iterate(mut self) -> Self {
		self.iterate = true;
		self
	}

	/// Set a target type
	pub fn with_target_type(mut self, target_type: ResourceType) -> Self {
		self.target_type = Some(target_type);
		self
	}
}

impl<'a> IntoQuery for IncludeParam<'a> {
	fn into_query(self) -> (String, String) {
		let mut name: String = if self.reverse { "_revInclude" } else { "_include" }.to_string();

		if self.iterate {
			name += ":iterate";
		}

		let mut value = format!("{}:{}", self.source_type.as_str(), self.field);

		if let Some(target_type) = self.target_type {
			value = format!("{}:{}", value, target_type.as_str())
		}

		(name, value)
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn number() {
		let number = NumberParam { comparator: Some(SearchComparator::Lt), value: "60" };
		assert_eq!(number.query_value(), "lt60".to_owned());
	}

	#[test]
	fn token() {
		let token = TokenParam::CodeInSystem { system: "system", code: "code", not: false };
		assert_eq!(token.query_value(), "system|code".to_owned());

		let token = TokenParam::CodeInAnySystem { code: "code", not: false };
		assert_eq!(token.query_value(), "code".to_owned());

		let token = TokenParam::CodeWithoutSystem { code: "code", not: false };
		assert_eq!(token.query_value(), "|code".to_owned());

		let token = TokenParam::InSystem { system: "system", not: false };
		assert_eq!(token.query_value(), "system|".to_owned());

		let token = TokenParam::OfType { type_system: None, type_code: None, value: Some("value") };
		assert_eq!(token.query_value(), "||value".to_owned());
	}

	#[test]
	fn quantity() {
		let quantity = QuantityParam { comparator: None, value: "1.0", system: None, code: None };
		assert_eq!(quantity.query_value(), "1.0".to_owned());

		let quantity =
			QuantityParam { comparator: None, value: "1.0", system: None, code: Some("g") };
		assert_eq!(quantity.query_value(), "1.0||g".to_owned());
	}

	#[test]
	fn missing() {
		let missing = MissingParam(true);
		assert_eq!(missing.query_value(), ("true".to_owned()));
	}

	#[test]
	fn include() {
		let include = IncludeParam {
			source_type: ResourceType::MedicationRequest,
			field: "encounter",
			target_type: None,
			iterate: false,
			reverse: false,
		};
		assert_eq!(
			include.into_query(),
			("_include".to_owned(), "MedicationRequest:encounter".to_owned())
		);

		let include = IncludeParam {
			source_type: ResourceType::Observation,
			field: "subject",
			target_type: Some(ResourceType::Patient),
			iterate: false,
			reverse: false,
		};
		assert_eq!(
			include.into_query(),
			("_include".to_owned(), "Observation:subject:Patient".to_owned())
		);

		let include = IncludeParam {
			source_type: ResourceType::Patient,
			field: "link",
			target_type: None,
			iterate: true,
			reverse: false,
		};
		assert_eq!(
			include.into_query(),
			("_include:iterate".to_owned(), "Patient:link".to_owned())
		);

		let include = IncludeParam {
			source_type: ResourceType::Encounter,
			field: "episode-of-care",
			target_type: None,
			iterate: false,
			reverse: true,
		};
		assert_eq!(
			include.into_query(),
			("_revInclude".to_owned(), "Encounter:episode-of-care".to_owned())
		);
	}
}
