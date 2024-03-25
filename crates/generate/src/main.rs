//! Code generation for FHIR types.
#![allow(clippy::expect_used, clippy::print_stdout)] // Just a generator crate.

mod generate;
mod model;
mod parse;

use std::fs;

use anyhow::{Context, Result};
use proc_macro2::TokenStream;

pub const BASE_FOLDER: &'static str = env!("CARGO_MANIFEST_DIR");

/// Generate code for all FHIR versions
pub fn main() -> Result<()> {
	println!("Parsing definitions..");
	let parsed = parse::parse_all();

	for (version, (codes, types, resources)) in parsed {
		println!("Generating {version} models..");

		let (generated_code, codes_map) = generate::generate_codes(codes)?;
		write_code_to_file(version, "codes", generated_code)?;

		let generated_code = generate::generate_types(types, &codes_map)?;
		write_code_to_file(version, "types", generated_code)?;

		let generated_code = generate::generate_resources(resources, &codes_map)?;
		write_code_to_file(version, "resources", generated_code)?;
	}

	Ok(())
}

fn write_code_to_file(version: &str, types: &str, code: TokenStream) -> Result<()> {
	let parsed = syn::parse2::<syn::File>(code).context("Parsing generated code to syn::File")?;
	let prettified = prettyplease::unparse(&parsed);

	let file = format!("{BASE_FOLDER}/../fhir-model/src/{version}/{types}/generated.rs");

	Ok(fs::write(file, prettified)?)
}
