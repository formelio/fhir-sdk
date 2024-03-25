mod codes;
mod structures;

use codes::*;
use structures::*;

use crate::model::codes::Code;
use crate::model::structures::Type;
use crate::model::{CodeSystemContentMode, PublicationStatus, StructureDefinitionKind};
use fhir_model::stu3;

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
