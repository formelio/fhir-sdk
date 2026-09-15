#![cfg(all(feature = "r5", feature = "builders"))]
#![allow(clippy::expect_used, clippy::unwrap_used, clippy::print_stdout)]
#![recursion_limit = "1024"]

mod json_compare;

use std::fs;

use fhir_model::{
	r5::{
		codes::{CatalogType, RequestIntent, RequestStatus},
		resources::{
			IdentifiableResource, NamedResource, Organization, Patient, RequestOrchestration,
			RequestOrchestrationAction, RequestOrchestrationActionTiming, Resource,
			WrongResourceType,
		},
		types::{CodeableConcept, Coding, Identifier, Reference},
	},
	Date, DateTime, ParsedReference,
};

/// Resource types excluded from generation via the generator's `IGNORED_TYPES`,
/// so their example files cannot be deserialized into [`Resource`].
const IGNORED_RESOURCE_TYPES: &[&str] = &[
	"Account",
	"AppointmentResponse",
	"Basic",
	"Claim",
	"ClaimResponse",
	"ClinicalImpression",
	"Communication",
	"CommunicationRequest",
	"Endpoint",
	"EnrollmentRequest",
	"EnrollmentResponse",
	"ExplanationOfBenefit",
	"FamilyMemberHistory",
	"Goal",
	"Group",
	"HealthcareService",
	"ImagingStudy",
	"Library",
	"List",
	"Measure",
	"PaymentNotice",
	"PaymentReconciliation",
	"ResearchStudy",
	"Schedule",
	"Slot",
	"Task",
	"VisionPrescription",
];

/// Whether `value` is, or recursively contains, a resource of an ignored type.
fn contains_ignored_resource(value: &Value) -> bool {
	match value {
		Value::Object(map) => {
			if let Some(Value::String(resource_type)) = map.get("resourceType") {
				if IGNORED_RESOURCE_TYPES.contains(&resource_type.as_str()) {
					return true;
				}
			}
			map.values().any(contains_ignored_resource)
		}
		Value::Array(array) => array.iter().any(contains_ignored_resource),
		_ => false,
	}
}
use serde_json::Value;

use self::json_compare::assert_fhir_json_equal;

#[test]
fn serialization_deserialization() {
	for entry in fs::read_dir(format!("{}/tests/r5-examples-json/", env!("CARGO_MANIFEST_DIR")))
		.expect("read dir")
	{
		let file = entry.expect("file entry").path();
		assert!(file.is_file());

		println!("Checking file `{}`..", file.display());

		let string = fs::read_to_string(&file).expect("reading file");
		let json: Value = serde_json::from_str(&string).expect("deserialize to value");
		// Skip examples that are, or contain (e.g. in `contained`), an ignored
		// resource type, since those can't be deserialized into `Resource`.
		if contains_ignored_resource(&json) {
			println!("Skipping `{}`: ignored resource type", file.display());
			continue;
		}
		let deserialized: Resource = serde_json::from_value(json.clone()).expect("deserializing");
		let serialized = serde_json::to_value(&deserialized).expect("serializing");
		assert_fhir_json_equal(&serialized, &json);
	}
}

#[test]
fn builder_works() {
	let _request_group = RequestOrchestration::builder()
		.status(RequestStatus::Active)
		.intent(RequestIntent::Order)
		.action(vec![Some(
			RequestOrchestrationAction::builder()
				.timing(RequestOrchestrationActionTiming::DateTime(DateTime::Date(Date::Year(
					2023,
				))))
				.code(vec![Some(
					CodeableConcept::builder()
						.coding(vec![Some(
							Coding::builder()
								.system("system".to_owned())
								.code("code".to_owned())
								.display("display".to_owned())
								.build()
								.unwrap(),
						)])
						.build()
						.unwrap(),
				)])
				.build()
				.unwrap(),
		)])
		.build()
		.unwrap();
}

#[test]
fn resource_conversion() {
	let patient = Patient::builder().build().unwrap();
	let resource: Resource = patient.into();
	let patient: Patient = resource.try_into().expect("It is a Patient resource");
	let resource: Resource = patient.into();
	let _patient: &Patient = (&resource).try_into().expect("It is a Patient resource");
	let result: Result<Organization, WrongResourceType> = resource.try_into();
	assert!(result.is_err());
}

#[test]
fn coding_concepts() {
	let code = CatalogType::_Custom("Test".to_owned());
	let coding = Coding::from(code.clone());
	assert!(coding.code.is_some());
	assert!(coding.system.is_some());
	let concept = CodeableConcept::from(code);
	assert_eq!(concept.coding.len(), 1);
	assert!(concept.text.is_some());
}

#[test]
fn resource_traits() {
	let ty = Patient::TYPE;
	let mut patient: Resource = Patient::builder().id("1".to_owned()).build().unwrap().into();
	assert_eq!(patient.resource_type(), ty);

	assert!(patient.as_base_resource().id().is_some());
	assert!(patient.as_domain_resource().is_some());

	patient.as_base_resource_mut().set_id(None);
	assert!(patient.as_base_resource().id().is_none());
}

#[test]
fn identifiable_resource() {
	let patient: Resource = Patient::builder()
		.identifier(vec![Some(
			Identifier::builder()
				.system("system".to_owned())
				.value("bla".to_owned())
				.build()
				.unwrap(),
		)])
		.build()
		.unwrap()
		.into();
	assert!(patient.as_identifiable_resource().is_some());

	let identifier = patient
		.as_identifiable_resource()
		.expect("Patient has identifiers")
		.identifier()
		.first()
		.and_then(Option::as_ref)
		.expect("We set one identifier");
	assert_eq!(identifier.system.as_deref(), Some("system"));
}

#[test]
fn identifier_search() {
	let patient = Patient::builder()
		.identifier(vec![
			Some(
				Identifier::builder()
					.system("system1".to_owned())
					.value("bla1".to_owned())
					.build()
					.unwrap(),
			),
			Some(
				Identifier::builder()
					.r#type(
						CodeableConcept::builder()
							.coding(vec![Some(
								Coding::builder()
									.system("system2".to_owned())
									.code("code2".to_owned())
									.build()
									.unwrap(),
							)])
							.build()
							.unwrap(),
					)
					.value("bla2".to_owned())
					.build()
					.unwrap(),
			),
		])
		.build()
		.unwrap();

	assert_eq!(patient.identifier_with_system("system1"), Some("bla1"));
	assert_eq!(patient.identifier_with_type("system2", "code2"), Some("bla2"));
}

#[test]
fn reference_parsing() {
	let reference = Reference::builder()
		.r#type("Encounter".to_owned())
		.reference("https://server.test/fhir/Encounter/1".to_owned())
		.build()
		.unwrap();
	let parsed = reference.parse().expect("parsing reference");
	assert_eq!(
		parsed,
		ParsedReference::Absolute {
			base_url: "https://server.test/fhir",
			resource_type: Some("Encounter"),
			id: Some("1"),
			version_id: None,
		}
	);

	let reference = Reference::builder()
		.r#type("Encounter".to_owned())
		.reference("https://server.test/fhir/Encounter/1/_history/1".to_owned())
		.build()
		.unwrap();
	let parsed = reference.parse().expect("parsing reference");
	assert_eq!(
		parsed,
		ParsedReference::Absolute {
			base_url: "https://server.test/fhir",
			resource_type: Some("Encounter"),
			id: Some("1"),
			version_id: Some("1"),
		}
	);

	let reference = Reference::builder()
		.r#type("Encounter".to_owned())
		.reference("Encounter/1".to_owned())
		.build()
		.unwrap();
	let parsed = reference.parse().expect("parsing reference");
	assert_eq!(
		parsed,
		ParsedReference::Relative { resource_type: "Encounter", id: "1", version_id: None }
	);

	let reference =
		Reference::builder().reference("Encounter/1/_history/1".to_owned()).build().unwrap();
	let parsed = reference.parse().expect("parsing reference");
	assert_eq!(
		parsed,
		ParsedReference::Relative { resource_type: "Encounter", id: "1", version_id: Some("1") }
	);

	let reference = Reference::builder()
		.r#type("Encounter".to_owned())
		.reference("#1".to_owned())
		.build()
		.unwrap();
	let parsed = reference.parse().expect("parsing reference");
	assert_eq!(parsed, ParsedReference::Local { id: "1" });

	let reference = Reference::builder()
		.r#type("Task".to_owned())
		.reference("http://not-fhir.test/1".to_owned())
		.build()
		.unwrap();
	let parsed = reference.parse().expect("parsing reference");
	assert_eq!(
		parsed,
		ParsedReference::Absolute {
			base_url: "http://not-fhir.test/1",
			resource_type: None,
			id: None,
			version_id: None,
		}
	);
}

#[test]
fn codeable_concept() {
	let concept = CodeableConcept::builder()
		.coding(vec![
			Some(
				Coding::builder()
					.system("system1".to_owned())
					.code("code1".to_owned())
					.build()
					.unwrap(),
			),
			Some(
				Coding::builder()
					.system("system2".to_owned())
					.code("code2".to_owned())
					.build()
					.unwrap(),
			),
			Some(
				Coding::builder()
					.system("system3".to_owned())
					.code("code3".to_owned())
					.build()
					.unwrap(),
			),
			Some(
				Coding::builder()
					.system("system1".to_owned())
					.code("code4".to_owned())
					.build()
					.unwrap(),
			),
		])
		.build()
		.unwrap();

	let mut codes1 = concept.codes_with_system("system1");
	assert_eq!(codes1.next(), Some("code1"));
	assert_eq!(codes1.next(), Some("code4"));
	assert_eq!(codes1.next(), None);
	let code3 = concept.code_with_system("system3");
	assert_eq!(code3, Some("code3"));
}
