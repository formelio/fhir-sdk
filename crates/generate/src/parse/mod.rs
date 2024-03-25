//! Parsing of the StructureDefinitions into the common model.
#![allow(clippy::fallible_impl_from)] // We want to panic on unexpected formats!

use crate::model::codes::Code;
use crate::model::structures::Type;
use crate::BASE_FOLDER;
use std::fs;

#[cfg(feature = "stu3")]
mod stu3;

#[cfg(feature = "r4b")]
mod r4b;

#[cfg(feature = "r5")]
mod r5;

pub(self) fn read_definitions(version: &str, types: &str) -> String {
	fs::read_to_string(format!("{BASE_FOLDER}/definitions/{version}/{types}.json"))
		.expect("Definitions not found")
}

/// Parses the types for all the enabled versions
pub fn parse_all() -> Vec<(&'static str, (Vec<Code>, Vec<Type>, Vec<Type>))> {
	let mut parsed = Vec::new();

	#[cfg(feature = "stu3")]
	{
		parsed.push(("stu3", stu3::parse_all()));
	}

	#[cfg(feature = "r4b")]
	{
		parsed.push(("r4b", r4b::parse_all()));
	}

	#[cfg(feature = "r5")]
	{
		parsed.push(("r5", r5::parse_all()));
	}

	parsed
}
