//! Resource definitions.

#[rustfmt::skip] // Too much for rustfmt
mod generated;
mod identifiable;

pub use generated::*;
pub use identifiable::*;

use crate::ParsedReference;

impl Bundle {
	/// Try to resolve an internal reference in the Bundle pretty much in accordance with
	/// the spec found at <https://www.hl7.org/fhir/stu3/bundle.html#references>.
	///
	/// Deviations from the spec:
	/// 1. When resolving versioned absolute references, if multiple entries are found with matching fullUrl
	///    and with a resource with a matching version ID, the spec says to fail the lookup. Instead, we
	///    return the first one we find.
	/// 2. When resolving relative references, the spec says to use the root of the fullUrl of the entry
	///    of the resource that contains the reference field as the root to stick the relative reference
	///    on before comparing it to fullUrls of other entries. Instead, we always use the value of the
	///    `base_url` parameter as the root for relative references.
	///
	/// Conditional references and external reference resolution are unsupported
	#[must_use] pub fn resolve_reference(
		&self,
		base_url: &str,
		reference: &ParsedReference,
	) -> Option<&Resource> {
		let full_url = reference.to_string();

		match *reference {
			// If the reference is local it has to be looked up in the resource itself
			ParsedReference::Local { .. } => {
				panic!("Can't resolve local (contained) references in Bundle");
			}
			// If the reference is an URN, look for an entry with a matching fullUrl
			ParsedReference::Absolute { .. } if full_url.starts_with("urn:") => self
				.entry
				.iter()
				.flatten()
				.find(|e| e.full_url.as_ref() == Some(&full_url))
				.and_then(|e| e.resource.as_ref()),
			// If the reference is absolute and versionless, look for an entry with a matching fullUrl
			// If multiple are found, use the one with the most recent lastUpdated value.
			ParsedReference::Absolute { version_id: None, .. } => self
				.entry
				.iter()
				.flatten()
				.filter(|e| e.full_url.as_ref() == Some(&full_url))
				.filter_map(|e| e.resource.as_ref())
				.max_by_key(|r| {
					r.as_base_resource().meta().as_ref().and_then(|m| m.last_updated.as_ref())
				}),
			// If the reference is absolute and versioned, look for an entry with the versionless version
			// of the reference as its fullUrl and whose resource has a matching version ID.
			// We go slightly off spec here because we return the first resource found if there are multiple
			// resources in the Bundle with the same fullUrl and version ID.
			ParsedReference::Absolute { version_id: Some(version_id), .. } => self
				.entry
				.iter()
				.flatten()
				.filter(|e| e.full_url.as_ref() == Some(&full_url))
				.filter_map(|e| e.resource.as_ref())
				.find(|r| {
					r.as_base_resource().meta().as_ref().and_then(|m| m.version_id.as_ref())
						== Some(&version_id.to_string())
				}),
			// If the reference is relative, use the `base_url` parameter to make it
			// absolute and resolve that.
			ParsedReference::Relative { .. } => {
				self.resolve_reference(base_url, &reference.with_base_url(base_url))
			}
		}
	}
}

#[cfg(test)]
mod tests {
	use std::str::FromStr;

	use super::super::codes::BundleType;
	use super::super::types::{Meta, Reference};
	use super::{Bundle, BundleEntry, Patient, Resource};
	use crate::Instant;

	const BASE_URL: &str = "http://my.fhir.server/fhir";

	#[test]
	fn resolve_reference_urns() {
		let patient_urn = "urn:uuid:8b3f7af2-a9ee-4cf0-a2a5-620da34b2c61";
		let reference = Reference::builder().reference(patient_urn.to_string()).build().unwrap();

		let patient = Patient::builder().build().unwrap();
		let bundle = test_bundle(&[(patient_urn, &patient)]);

		assert_eq!(
			bundle.resolve_reference(BASE_URL, &reference.parse().unwrap()),
			Some(&Resource::Patient(patient)),
		);

		let invalid_reference = Reference::builder()
			.reference("urn:uuid:c44b624d-890f-4902-9cc6-7920fcf20b33".to_string())
			.build()
			.unwrap();
		assert_eq!(bundle.resolve_reference(BASE_URL, &invalid_reference.parse().unwrap()), None);
	}

	#[test]
	fn resolve_reference_absolute_versionless_resolves() {
		let patient_url = format!("{BASE_URL}/Patient/123");

		let patient_reference =
			Reference::builder().reference(patient_url.to_string()).build().unwrap();
		let patient = Patient::builder().build().unwrap();

		let bundle = test_bundle(&[(&patient_url, &patient)]);

		assert_eq!(
			bundle.resolve_reference(BASE_URL, &patient_reference.parse().unwrap()),
			Some(&Resource::Patient(patient)),
			"resolves absolute reference {} to a Patient in the Bundle",
			patient_reference.reference.as_ref().unwrap()
		);

		let invalid_reference = Reference::builder()
			.reference(format!("{BASE_URL}/Patient/789").to_string())
			.build()
			.unwrap();
		assert_eq!(
			bundle.resolve_reference(BASE_URL, &invalid_reference.parse().unwrap()),
			None,
			"does not resolve absolute reference {} to a Patient resource not in the Bundle",
			invalid_reference.reference.as_ref().unwrap()
		);
	}

	#[test]
	fn resolve_reference_absolute_versionless_resolves_latest_version() {
		let patient_url = format!("{BASE_URL}/Patient/123");

		let old_timestamp = Instant::from_str("2024-02-20T18:00:00+02:00").unwrap();
		let new_timestamp = Instant::from_str("2024-02-27T12:00:00+02:00").unwrap();

		let old_patient = Patient::builder()
			.meta(Meta::builder().last_updated(old_timestamp).build().unwrap())
			.build()
			.unwrap();

		let new_patient = Patient::builder()
			.meta(Meta::builder().last_updated(new_timestamp).build().unwrap())
			.build()
			.unwrap();

		let bundle = test_bundle(&[(&patient_url, &old_patient), (&patient_url, &new_patient)]);

		let patient_reference =
			Reference::builder().reference(patient_url.to_string()).build().unwrap();

		assert_eq!(
			bundle.resolve_reference(BASE_URL, &patient_reference.parse().unwrap()),
			Some(&Resource::Patient(new_patient)),
			"resolving a versionless absolute reference {} in a Bundle with multiple Patients with the same fullUrl finds the Patient with the latest lastUpdated value",
			patient_reference.reference.as_ref().unwrap()
		);

		let bundle = test_bundle(&[(&patient_url, &old_patient)]);

		assert_eq!(
			bundle.resolve_reference(BASE_URL, &patient_reference.parse().unwrap()),
			Some(&Resource::Patient(old_patient)),
			"resolving a versionless absolute reference {} in a Bundle with just an older Patient finds that Patient",
			patient_reference.reference.as_ref().unwrap()
		);
	}

	#[test]
	fn resolve_reference_absolute_versioned_resolves() {
		let patient1_url = format!("{BASE_URL}/Patient/123/_history/1");
		let patient2_url = format!("{BASE_URL}/Patient/123/_history/2");

		let patient1 = Patient::builder()
			.meta(Meta::builder().version_id("1".to_string()).build().unwrap())
			.active(true)
			.build()
			.unwrap();
		let patient2 = Patient::builder()
			.meta(Meta::builder().version_id("2".to_string()).build().unwrap())
			.active(false)
			.build()
			.unwrap();

		let bundle = test_bundle(&[(&patient1_url, &patient1), (&patient2_url, &patient2)]);

		let patient1_reference =
			Reference::builder().reference(patient1_url.to_string()).build().unwrap();

		assert_eq!(
			bundle.resolve_reference(BASE_URL, &patient1_reference.parse().unwrap()),
			Some(&Resource::Patient(patient1)),
			"resolves absolute reference {} to Patient version 1 in the Bundle",
			patient1_reference.reference.as_ref().unwrap()
		);

		let patient2_reference =
			Reference::builder().reference(patient2_url.to_string()).build().unwrap();

		assert_eq!(
			bundle.resolve_reference(BASE_URL, &patient2_reference.parse().unwrap()),
			Some(&Resource::Patient(patient2)),
			"resolves absolute versioned reference {} to Patient version 2 in the Bundle",
			patient2_reference.reference.as_ref().unwrap()
		);

		let patient3_reference = Reference::builder()
			.reference(format!("{BASE_URL}/Patient/123/_history/3").to_string())
			.build()
			.unwrap();

		assert_eq!(
			bundle.resolve_reference(BASE_URL, &patient3_reference.parse().unwrap()),
			None,
			"does not resolve absolute versioned reference {} to version of Patient resource not in the Bundle",
			patient3_reference.reference.as_ref().unwrap()
		);
	}

	#[test]
	fn resolve_reference_absolute_versioned_multiple_offspec_resolves() {
		let patient_url = format!("{BASE_URL}/Patient/123/_history/1");

		let patient1a = Patient::builder()
			.meta(Meta::builder().version_id("1".to_string()).build().unwrap())
			.active(true)
			.build()
			.unwrap();

		let patient1b = Patient::builder()
			.meta(Meta::builder().version_id("1".to_string()).build().unwrap())
			.active(false)
			.build()
			.unwrap();

		let bundle = test_bundle(&[(&patient_url, &patient1a), (&patient_url, &patient1b)]);

		let patient_reference =
			Reference::builder().reference(patient_url.to_string()).build().unwrap();

		assert_eq!(
			bundle.resolve_reference(BASE_URL, &patient_reference.parse().unwrap()),
			Some(&Resource::Patient(patient1a)),
			"resolves absolute versioned reference {} in Bundle with multiple matches to the first one found (off-spec)",
			patient_reference.reference.as_ref().unwrap()
		);
	}

	#[test]
	fn resolve_reference_relative_resolves() {
		let patient_relative_url = "Patient/123".to_string();
		let patient_url = format!("{BASE_URL}/{patient_relative_url}");

		let patient = Patient::builder().build().unwrap();

		let bundle = test_bundle(&[(&patient_url, &patient)]);

		let patient_reference =
			Reference::builder().reference(patient_relative_url.to_string()).build().unwrap();

		assert_eq!(
			bundle.resolve_reference(BASE_URL, &patient_reference.parse().unwrap()),
            Some(&Resource::Patient(patient)),
			"resolves relative unversioned reference {} to Patient with same base URL the same as with an absolute reference",
			patient_reference.reference.as_ref().unwrap()
		);

		assert_eq!(
			bundle.resolve_reference("http://other.base.url/fhir", &patient_reference.parse().unwrap()),
            None,
			"does not resolves relative unversioned reference {} to Patient with different base URL",
			patient_reference.reference.as_ref().unwrap()
		);
	}

	#[test]
	fn resolve_reference_relative_resolves_versioned() {
		let patient1_relative_url = "Patient/123/_history/1".to_string();
		let patient1_url = format!("{BASE_URL}/{patient1_relative_url}");

		let patient2_relative_url = "Patient/123/_history/2".to_string();
		let patient2_url = format!("{BASE_URL}/{patient2_relative_url}");

		let patient1 = Patient::builder()
			.meta(Meta::builder().version_id("1".to_string()).build().unwrap())
			.active(true)
			.build()
			.unwrap();
		let patient2 = Patient::builder()
			.meta(Meta::builder().version_id("2".to_string()).build().unwrap())
			.active(false)
			.build()
			.unwrap();

		let bundle = test_bundle(&[(&patient1_url, &patient1), (&patient2_url, &patient2)]);

		let patient1_reference =
			Reference::builder().reference(patient1_relative_url.to_string()).build().unwrap();

		assert_eq!(
			bundle.resolve_reference(BASE_URL, &patient1_reference.parse().unwrap()),
			Some(&Resource::Patient(patient1)),
			"resolves relative versioned reference {} to Patient version 1 in the Bundle",
			patient1_reference.reference.as_ref().unwrap()
		);

		let patient2_reference =
			Reference::builder().reference(patient2_relative_url.to_string()).build().unwrap();

		assert_eq!(
			bundle.resolve_reference(BASE_URL, &patient2_reference.parse().unwrap()),
			Some(&Resource::Patient(patient2)),
			"resolves relative versioned reference {} to Patient version 2 in the Bundle",
			patient2_reference.reference.as_ref().unwrap()
		);

		let patient3_reference =
			Reference::builder().reference("Patient/123/_history/3".to_string()).build().unwrap();

		assert_eq!(
			bundle.resolve_reference(BASE_URL, &patient3_reference.parse().unwrap()),
			None,
			"does not resolve relative versioned reference {} to version of Patient resource not in the Bundle",
			patient3_reference.reference.as_ref().unwrap()
		);

		assert_eq!(
			bundle.resolve_reference(
				"http://other.base.url/fhir",
				&patient1_reference.parse().unwrap()
			),
			None,
			"does not resolve relative versioned reference {} with different base URL",
			patient1_reference.reference.as_ref().unwrap()
		);
	}

	fn test_bundle(patients: &[(&str, &Patient)]) -> Bundle {
		let mut entries = Vec::new();

		for (full_url, patient) in patients {
			let entry = BundleEntry::builder()
				.full_url((*full_url).to_string())
				.resource((*patient).clone().into())
				.build()
				.unwrap();

			entries.push(Some(entry));
		}

		Bundle::builder().r#type(BundleType::Searchset).entry(entries).build().unwrap()
	}
}
