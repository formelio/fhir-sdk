//! Run code generation.
#![allow(clippy::print_stdout)]

use anyhow::Result;

fn main() -> Result<()> {
	println!("Generating STU3 models..");
	generate::generate_code("stu3")?;

	println!("Done.");
	Ok(())
}
