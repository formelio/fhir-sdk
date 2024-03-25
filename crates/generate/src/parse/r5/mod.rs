mod codes;
mod structures;

use fhir_model::r5;

pub use codes::*;
pub use structures::*;

use crate::model::codes::Code;
use crate::model::structures::Type;
use crate::model::{CodeSystemContentMode, PublicationStatus, StructureDefinitionKind};

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
		parse_types("types");
	}

	#[test]
	fn parses_resources() {
		parse_types("resources");
	}
}
