use fhir_model::r4b;

mod codes;
mod structures;

pub use codes::*;
pub use structures::*;

use crate::model::codes::Code;
use crate::model::structures::Type;
use crate::model::{CodeSystemContentMode, PublicationStatus, StructureDefinitionKind};

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

pub fn parse_all() -> (Vec<Code>, Vec<Type>, Vec<Type>) {
	(parse_codes(), parse_types("profiles-types"), parse_types("profiles-resources"))
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn parses_codes() {
		parse_codes();
	}

	#[test]
	fn parses_types() {
		parse_types("profiles-types");
	}

	#[test]
	fn parses_resources() {
		parse_types("profiles-resources");
	}
}
