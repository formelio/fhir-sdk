//! Type definitions.

#[rustfmt::skip] // Too much for rustfmt
mod generated;

use itertools::Itertools;
use std::fmt::Display;

pub use generated::*;

use super::resources::ResourceType;
use crate::ParsedReference;

impl CodeableConcept {
	/// Get all codes matching a specific system.
	pub fn codes_with_system<'a, 'b>(
		&'a self,
		system: &'b str,
	) -> impl Iterator<Item = &'a str> + Send + 'b
	where
		'a: 'b,
	{
		self.coding
			.iter()
			.flatten()
			.filter(|coding| coding.system.as_deref() == Some(system))
			.filter_map(|coding| coding.code.as_deref())
	}

	/// Get the first code matching a specific system.
	#[must_use]
	pub fn code_with_system<'a>(&'a self, system: &str) -> Option<&'a str> {
		self.codes_with_system(system).next()
	}
}

impl Display for CodeableConcept {
	/// Finds the right display value for a CodeableConcept.
	///
	/// Arguably opinionated, but mostly in accordance with [the spec](https://www.hl7.org/fhir/stu3/datatypes.html#codeableconcept)
	///
	/// Uses the following steps to find the appropriate display value
	/// 1. If any `coding` fields are marked as user selected through `userSelected`, use their `display` value. If multiple
	///    are found, comma separate them.
	/// 2. Otherwise, if a `text` field is present, use that.
	/// 3. Otherwise, use the first `coding` field with a `display` value.
	/// 4. Otherwise, return an empty string
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		let user_selected: Vec<_> =
			self.coding.iter().flatten().filter(|c| c.user_selected.unwrap_or_default()).collect();

		let display = if !user_selected.is_empty() {
			user_selected.iter().join(", ")
		} else if let Some(text) = &self.text {
			text.to_string()
		} else {
			self.coding.iter().flatten().find_map(|c| c.display.clone()).unwrap_or_default()
		};

		write!(f, "{display}")
	}
}

impl Display for Coding {
	/// Uses the `Coding.display` value if present, otherwise an empty string
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		write!(f, "{}", self.display.as_deref().unwrap_or_default())
	}
}

impl Display for HumanName {
	/// A very basic display implementation for [HumanName]. Uses `HumanName.name` if available. If not,
	/// appends the first `HumanName.given` and the `HumanName.family` separated by a space. Note that
	/// the output could be e.g. an empty string or just initials or just a last name.
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		if let Some(ref text) = self.text {
			write!(f, "{text}")
		} else {
			let name = format!(
				"{} {}",
				self.given.iter().flatten().next().cloned().as_deref().unwrap_or_default(),
				self.family.as_deref().unwrap_or_default()
			);

			write!(f, "{}", name.trim())
		}
	}
}

impl Reference {
	/// Parse the [`Reference`] into a [`ParsedReference`]. Returns `None` if
	/// the `reference` field is empty.
	#[must_use]
	pub fn parse(&self) -> Option<ParsedReference<'_>> {
		let url = self.reference.as_ref()?;
		Some(ParsedReference::new::<ResourceType>(url))
	}
}

impl From<ParsedReference<'_>> for Reference {
	fn from(parsed: ParsedReference<'_>) -> Self {
		let reference = ReferenceInner {
			id: None,
			extension: Vec::new(),
			reference: Some(parsed.to_string()),
			reference_ext: None,
			identifier: None,
			identifier_ext: None,
			display: None,
			display_ext: None,
		};

		reference.into()
	}
}

#[cfg(test)]
mod tests {
	use super::super::types::{CodeableConcept, Coding};

	#[test]
	fn display_coding() {
		let display = "test";
		let with_display = Coding::builder().display(display.to_string()).build().unwrap();
		assert_eq!(with_display.to_string(), display.to_string());

		let without_display = Coding::builder().build().unwrap();
		assert_eq!(without_display.to_string(), String::new());
	}

	#[test]
	fn display_codeable_concept() {
		let display = "test";

		let user_selected =
			Coding::builder().display(display.to_string()).user_selected(true).build().unwrap();
		let with_user_selected =
			CodeableConcept::builder().coding(vec![Some(user_selected)]).build().unwrap();
		assert_eq!(with_user_selected.to_string(), display.to_string());

		let with_text = CodeableConcept::builder().text(display.to_string()).build().unwrap();
		assert_eq!(with_text.to_string(), display);

		let not_user_selected = Coding::builder().display(display.to_string()).build().unwrap();
		let without_user_selected =
			CodeableConcept::builder().coding(vec![Some(not_user_selected)]).build().unwrap();
		assert_eq!(without_user_selected.to_string(), display.to_string());

		let without_display = CodeableConcept::builder().build().unwrap();
		assert_eq!(without_display.to_string(), String::new());
	}
}
