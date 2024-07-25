//! Parsing of the StructureDefinitions into the common model.
#![allow(clippy::fallible_impl_from)] // We want to panic on unexpected formats!

pub mod codes;
pub mod params;
pub mod structures;

use fhir_model::{r4b, r5, stu3};

use crate::model::params::SearchComparator;
use crate::model::{
	CodeSystemContentMode, PublicationStatus, SearchParamType, StructureDefinitionKind,
};

impl From<stu3::codes::PublicationStatus> for PublicationStatus {
	fn from(value: stu3::codes::PublicationStatus) -> Self {
		match value {
			stu3::codes::PublicationStatus::Active => Self::Active,
			stu3::codes::PublicationStatus::Draft => Self::Draft,
			stu3::codes::PublicationStatus::Retired => Self::Retired,
			stu3::codes::PublicationStatus::Unknown => Self::Unknown,
		}
	}
}

impl From<r4b::codes::PublicationStatus> for PublicationStatus {
	fn from(value: r4b::codes::PublicationStatus) -> Self {
		match value {
			r4b::codes::PublicationStatus::Active => Self::Active,
			r4b::codes::PublicationStatus::Draft => Self::Draft,
			r4b::codes::PublicationStatus::Retired => Self::Retired,
			r4b::codes::PublicationStatus::Unknown => Self::Unknown,
		}
	}
}

impl From<r5::codes::PublicationStatus> for PublicationStatus {
	fn from(value: r5::codes::PublicationStatus) -> Self {
		match value {
			r5::codes::PublicationStatus::Active => Self::Active,
			r5::codes::PublicationStatus::Draft => Self::Draft,
			r5::codes::PublicationStatus::Retired => Self::Retired,
			r5::codes::PublicationStatus::Unknown => Self::Unknown,
		}
	}
}

impl From<stu3::codes::StructureDefinitionKind> for StructureDefinitionKind {
	fn from(value: stu3::codes::StructureDefinitionKind) -> Self {
		match value {
			stu3::codes::StructureDefinitionKind::ComplexType => Self::ComplexType,
			stu3::codes::StructureDefinitionKind::Logical => Self::Logical,
			stu3::codes::StructureDefinitionKind::PrimitiveType => Self::PrimitiveType,
			stu3::codes::StructureDefinitionKind::Resource => Self::Resource,
		}
	}
}

impl From<r4b::codes::StructureDefinitionKind> for StructureDefinitionKind {
	fn from(value: r4b::codes::StructureDefinitionKind) -> Self {
		match value {
			r4b::codes::StructureDefinitionKind::ComplexType => Self::ComplexType,
			r4b::codes::StructureDefinitionKind::Logical => Self::Logical,
			r4b::codes::StructureDefinitionKind::PrimitiveType => Self::PrimitiveType,
			r4b::codes::StructureDefinitionKind::Resource => Self::Resource,
		}
	}
}

impl From<r5::codes::StructureDefinitionKind> for StructureDefinitionKind {
	fn from(value: r5::codes::StructureDefinitionKind) -> Self {
		match value {
			r5::codes::StructureDefinitionKind::ComplexType => Self::ComplexType,
			r5::codes::StructureDefinitionKind::Logical => Self::Logical,
			r5::codes::StructureDefinitionKind::PrimitiveType => Self::PrimitiveType,
			r5::codes::StructureDefinitionKind::Resource => Self::Resource,
		}
	}
}

impl From<stu3::codes::CodeSystemContentMode> for CodeSystemContentMode {
	fn from(value: stu3::codes::CodeSystemContentMode) -> Self {
		match value {
			stu3::codes::CodeSystemContentMode::Complete => Self::Complete,
			stu3::codes::CodeSystemContentMode::Example => Self::Example,
			stu3::codes::CodeSystemContentMode::Fragment => Self::Fragment,
			stu3::codes::CodeSystemContentMode::NotPresent => Self::NotPresent,
		}
	}
}

impl From<r4b::codes::CodeSystemContentMode> for CodeSystemContentMode {
	fn from(value: r4b::codes::CodeSystemContentMode) -> Self {
		match value {
			r4b::codes::CodeSystemContentMode::Complete => Self::Complete,
			r4b::codes::CodeSystemContentMode::Example => Self::Example,
			r4b::codes::CodeSystemContentMode::Fragment => Self::Fragment,
			r4b::codes::CodeSystemContentMode::NotPresent => Self::NotPresent,
			r4b::codes::CodeSystemContentMode::Supplement => Self::Supplement,
		}
	}
}

impl From<r5::codes::CodeSystemContentMode> for CodeSystemContentMode {
	fn from(value: r5::codes::CodeSystemContentMode) -> Self {
		match value {
			r5::codes::CodeSystemContentMode::Complete => Self::Complete,
			r5::codes::CodeSystemContentMode::Example => Self::Example,
			r5::codes::CodeSystemContentMode::Fragment => Self::Fragment,
			r5::codes::CodeSystemContentMode::NotPresent => Self::NotPresent,
			r5::codes::CodeSystemContentMode::Supplement => Self::Supplement,
		}
	}
}

impl From<r5::codes::SearchParamType> for SearchParamType {
	fn from(value: r5::codes::SearchParamType) -> Self {
		match value {
			r5::codes::SearchParamType::Composite => SearchParamType::Composite,
			r5::codes::SearchParamType::Date => SearchParamType::Date,
			r5::codes::SearchParamType::Number => SearchParamType::Number,
			r5::codes::SearchParamType::Quantity => SearchParamType::Quantity,
			r5::codes::SearchParamType::Reference => SearchParamType::Reference,
			r5::codes::SearchParamType::Special => SearchParamType::Special,
			r5::codes::SearchParamType::String => SearchParamType::String,
			r5::codes::SearchParamType::Token => SearchParamType::Token,
			r5::codes::SearchParamType::Uri => SearchParamType::Uri,
		}
	}
}

impl From<r4b::codes::SearchParamType> for SearchParamType {
	fn from(value: r4b::codes::SearchParamType) -> Self {
		match value {
			r4b::codes::SearchParamType::Composite => SearchParamType::Composite,
			r4b::codes::SearchParamType::Date => SearchParamType::Date,
			r4b::codes::SearchParamType::Number => SearchParamType::Number,
			r4b::codes::SearchParamType::Quantity => SearchParamType::Quantity,
			r4b::codes::SearchParamType::Reference => SearchParamType::Reference,
			r4b::codes::SearchParamType::Special => SearchParamType::Special,
			r4b::codes::SearchParamType::String => SearchParamType::String,
			r4b::codes::SearchParamType::Token => SearchParamType::Token,
			r4b::codes::SearchParamType::Uri => SearchParamType::Uri,
		}
	}
}

impl From<stu3::codes::SearchParamType> for SearchParamType {
	fn from(value: stu3::codes::SearchParamType) -> Self {
		match value {
			stu3::codes::SearchParamType::Composite => SearchParamType::Composite,
			stu3::codes::SearchParamType::Date => SearchParamType::Date,
			stu3::codes::SearchParamType::Number => SearchParamType::Number,
			stu3::codes::SearchParamType::Quantity => SearchParamType::Quantity,
			stu3::codes::SearchParamType::Reference => SearchParamType::Reference,
			stu3::codes::SearchParamType::String => SearchParamType::String,
			stu3::codes::SearchParamType::Token => SearchParamType::Token,
			stu3::codes::SearchParamType::Uri => SearchParamType::Uri,
		}
	}
}

impl From<r5::codes::SearchComparator> for SearchComparator {
	fn from(value: r5::codes::SearchComparator) -> Self {
		match value {
			r5::codes::SearchComparator::Eq => SearchComparator::Eq,
			r5::codes::SearchComparator::Ne => SearchComparator::Ne,
			r5::codes::SearchComparator::Gt => SearchComparator::Gt,
			r5::codes::SearchComparator::Lt => SearchComparator::Lt,
			r5::codes::SearchComparator::Ge => SearchComparator::Ge,
			r5::codes::SearchComparator::Le => SearchComparator::Le,
			r5::codes::SearchComparator::Sa => SearchComparator::Sa,
			r5::codes::SearchComparator::Eb => SearchComparator::Eb,
			r5::codes::SearchComparator::Ap => SearchComparator::Ap,
		}
	}
}

impl From<r4b::codes::SearchComparator> for SearchComparator {
	fn from(value: r4b::codes::SearchComparator) -> Self {
		match value {
			r4b::codes::SearchComparator::Eq => SearchComparator::Eq,
			r4b::codes::SearchComparator::Ne => SearchComparator::Ne,
			r4b::codes::SearchComparator::Gt => SearchComparator::Gt,
			r4b::codes::SearchComparator::Lt => SearchComparator::Lt,
			r4b::codes::SearchComparator::Ge => SearchComparator::Ge,
			r4b::codes::SearchComparator::Le => SearchComparator::Le,
			r4b::codes::SearchComparator::Sa => SearchComparator::Sa,
			r4b::codes::SearchComparator::Eb => SearchComparator::Eb,
			r4b::codes::SearchComparator::Ap => SearchComparator::Ap,
		}
	}
}

impl From<stu3::codes::SearchComparator> for SearchComparator {
	fn from(value: stu3::codes::SearchComparator) -> Self {
		match value {
			stu3::codes::SearchComparator::Eq => SearchComparator::Eq,
			stu3::codes::SearchComparator::Ne => SearchComparator::Ne,
			stu3::codes::SearchComparator::Gt => SearchComparator::Gt,
			stu3::codes::SearchComparator::Lt => SearchComparator::Lt,
			stu3::codes::SearchComparator::Ge => SearchComparator::Ge,
			stu3::codes::SearchComparator::Le => SearchComparator::Le,
			stu3::codes::SearchComparator::Sa => SearchComparator::Sa,
			stu3::codes::SearchComparator::Eb => SearchComparator::Eb,
			stu3::codes::SearchComparator::Ap => SearchComparator::Ap,
		}
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn parse_types_from_structure_definitions() {
		let included_types = include_str!("../../definitions/stu3/profiles-types.json");
		let _types = structures::parse_stu3(included_types);
		let included_resources = include_str!("../../definitions/stu3/profiles-resources.json");
		let _types = structures::parse_stu3(included_resources);

		let included_types = include_str!("../../definitions/r4b/profiles-types.json");
		let _types = structures::parse_r4b(included_types);
		let included_resources = include_str!("../../definitions/r4b/profiles-resources.json");
		let _types = structures::parse_r4b(included_resources);

		let included_types = include_str!("../../definitions/r5/profiles-types.json");
		let _types = structures::parse_r5(included_types);
		let included_resources = include_str!("../../definitions/r5/profiles-resources.json");
		let _types = structures::parse_r5(included_resources);
	}

	#[test]
	fn parse_value_sets_from_code_systems() {
		let included = include_str!("../../definitions/stu3/valuesets.json");
		let _codes = codes::parse_stu3(included);
		let included = include_str!("../../definitions/r4b/valuesets.json");
		let _codes = codes::parse_r4b(included);
		let included = include_str!("../../definitions/r5/valuesets.json");
		let _codes = codes::parse_r5(included);
	}
}
