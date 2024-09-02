use fhir_model::r5::resources::{Bundle, DomainResource, Resource, };
use fhir_model::ParsedReference;
use reqwest::Url;

pub(super) fn populate_reference_targets(base_url: &Url, resource: &mut Resource) {
	if let Resource::Bundle(bundle) = resource {
		let lookup = bundle.clone();

		for entry in bundle.entry.iter_mut().flatten() {
			if let Some(resource) = entry.resource.as_mut() {
				if let Some(domain_resource) = resource.as_domain_resource_mut() {
					populate_reference_targets_internal(
						base_url,
						domain_resource,
						Some(&lookup),
						None,
					);
				}
			}
		}
	} else if let Some(domain_resource) = resource.as_domain_resource_mut() {
		populate_reference_targets_internal(base_url, domain_resource, None, None)
	}
}

pub(super) fn populate_reference_targets_internal(
	base_url: &Url,
	resource: &mut dyn DomainResource,
	bundle: Option<&Bundle>,
	sibling_contained: Option<&Vec<Resource>>,
) {
	// If this resource is a contained resource, sibling_contained will be set so we use
	// that for lookup. Otherwise, this resource is not contained and we use its contained
	// resources for lookup
	let contained_lookup = sibling_contained.unwrap_or_else(|| resource.contained()).clone();

	for field in resource.lookup_references() {
		if let Some(reference) = field.reference().clone().parse() {
			let target = match reference {
				ParsedReference::Local { id } => contained_lookup
					.iter()
					.find(|c| c.as_base_resource().id() == &Some(id.to_string())),
				other if bundle.is_some() => {
					bundle.unwrap().resolve_reference(base_url.as_str(), &other)
				}
				_ => None,
			};

			if let Some(target) = target {
				if field.set_target(target.clone()).is_err() {
					tracing::warn!("Reference {} in Bundled resource refers to resource of unsupported type {}", reference.to_string(), target.resource_type());
				}
			} else {
				tracing::debug!(
					"Unable to resolve reference {} in Bundled resource",
					reference.to_string(),
				);
			}
		}
	}

	let contained = resource.contained_mut();

	if !contained.is_empty() && sibling_contained.is_some() {
		tracing::warn!("Nested contained resources found while populating reference targets");
	}

	// Populate references in all contained resources as well
	for c in contained {
		if let Some(contained_domain_resource) = c.as_domain_resource_mut() {
			// We pass on the same sibling_contained lookup value since contained resources
			// are able to refer to siblings but they're not allowed to be nested
			populate_reference_targets_internal(
				base_url,
				contained_domain_resource,
				bundle,
				Some(&contained_lookup),
			);
		}
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	use fhir_model::r5::{
		codes::{BundleType, ObservationStatus},
		local_reference_to, reference_to,
		resources::{
			Bundle, BundleEntry, Observation, ObservationSubjectReferenceTarget, Patient,
			PatientGeneralPractitionerReferenceTarget, Practitioner, Resource,
		},
		types::{CodeableConcept, Reference},
	};
	use uuid::Uuid;

	use crate::Url;

	fn base_url() -> Url {
		Url::parse("http://my.fhir.server/fhir").unwrap()
	}

	#[test]
	fn populates_references_local() {
		let base_url = base_url();

		let practitioner = Practitioner::builder().id(Uuid::new_v4().to_string()).build().unwrap();
		let practitioner_ref = local_reference_to(&practitioner).unwrap();

		let patient = Patient::builder()
			.id(Uuid::new_v4().to_string())
			.general_practitioner(vec![Some(Reference::from(practitioner_ref).into())])
			.build()
			.unwrap();
		let patient_ref = local_reference_to(&patient).unwrap();

		let mut observation = test_observation();
		observation.contained = vec![patient.clone().into(), practitioner.clone().into()];
		observation.subject = Some(patient_ref.into());

		populate_reference_targets_internal(&base_url, &mut observation, None, None);

		// Check if the subject field points to the patient
		assert_eq!(
			observation.subject.as_ref().and_then(|s| s.target.as_ref()),
			Some(&Box::new(ObservationSubjectReferenceTarget::Patient(patient)))
		);

		let general_practitioner = observation
			.contained
			.iter()
			.find_map(|c| match c {
				Resource::Patient(p) => Some(p),
				_ => None,
			})
			.and_then(|p| p.general_practitioner.iter().flatten().next())
			.unwrap();

		// Check if the contained patient's generalPractitioner field points to the Practitioner
		assert_eq!(
			general_practitioner.target.as_ref(),
			Some(&Box::new(PatientGeneralPractitionerReferenceTarget::Practitioner(practitioner))),
		);
	}

	#[test]
	fn populates_references_urn() {
		let base_url = base_url();

		let patient_urn = Uuid::new_v4().urn().to_string();
		let subject = Reference::builder().reference(patient_urn.clone()).build().unwrap();

		let patient = test_patient();

		let mut observation = test_observation();
		observation.subject = Some(subject.into());

		let observation_ref = reference_to(&observation).unwrap();
		let observation_full_url =
			observation_ref.parse().unwrap().with_base_url(base_url.as_str()).to_string();

		let bundle = test_bundle(vec![
			(&patient_urn, patient.clone().into()),
			(&observation_full_url, observation.clone().into()),
		]);

		populate_reference_targets_internal(&base_url, &mut observation, Some(&bundle), None);

		assert_eq!(
			observation.subject.as_ref().and_then(|s| s.target.as_ref()),
			Some(&Box::new(ObservationSubjectReferenceTarget::Patient(patient)))
		);
	}

	#[test]
	fn populates_references_absolute() {
		let base_url = base_url();

		let patient = test_patient();
		let patient_ref = reference_to(&patient).unwrap();
		let patient_full_url = patient_ref.parse().unwrap().with_base_url(base_url.as_str());

		let mut observation = test_observation();
		observation.subject = Some(Reference::from(patient_full_url).into());

		let observation_ref = reference_to(&observation).unwrap();
		let observation_full_url =
			observation_ref.parse().unwrap().with_base_url(base_url.as_str());

		let bundle = test_bundle(vec![
			(&patient_full_url.to_string(), patient.clone().into()),
			(&observation_full_url.to_string(), observation.clone().into()),
		]);

		populate_reference_targets_internal(&base_url, &mut observation, Some(&bundle), None);

		assert_eq!(
			observation.subject.as_ref().and_then(|s| s.target.as_ref()),
			Some(&Box::new(ObservationSubjectReferenceTarget::Patient(patient)))
		);
	}

	#[test]
	fn populates_references_relative() {
		let base_url = base_url();

		let patient = test_patient();
		let patient_ref = reference_to(&patient).unwrap();
		let patient_full_url = patient_ref.parse().unwrap().with_base_url(base_url.as_str());

		let mut observation = test_observation();
		observation.subject = Some(Reference::from(patient_ref.clone()).into());

		let observation_ref = reference_to(&observation).unwrap();
		let observation_full_url =
			observation_ref.parse().unwrap().with_base_url(base_url.as_str());

		let bundle = test_bundle(vec![
			(&patient_full_url.to_string(), patient.clone().into()),
			(&observation_full_url.to_string(), observation.clone().into()),
		]);

		populate_reference_targets_internal(&base_url, &mut observation, Some(&bundle), None);

		assert_eq!(
			observation.subject.as_ref().and_then(|s| s.target.as_ref()),
			Some(&Box::new(ObservationSubjectReferenceTarget::Patient(patient)))
		);
	}

	fn test_bundle(resources: Vec<(&str, Resource)>) -> Bundle {
		let mut bundle = Bundle::builder().r#type(BundleType::Collection).build().unwrap();

		for (full_url, resource) in resources {
			let entry = BundleEntry::builder()
				.full_url(full_url.to_string())
				.resource(resource)
				.build()
				.unwrap();

			bundle.entry.push(Some(entry));
		}

		bundle
	}

	fn test_patient() -> Patient {
		Patient::builder().id(Uuid::new_v4().to_string()).build().unwrap()
	}

	fn test_observation() -> Observation {
		Observation::builder()
			.id(Uuid::new_v4().to_string())
			.status(ObservationStatus::Final)
			.code(CodeableConcept::builder().build().unwrap())
			.build()
			.unwrap()
	}
}
