#![cfg(all(feature = "stu3", feature = "builders", feature = "client"))]
#![allow(clippy::expect_used, clippy::unwrap_used, clippy::print_stdout)]
#![recursion_limit = "1024"]

mod common;

use std::{env, str::FromStr};

use eyre::Result;
use fhir_sdk::{
	client::{
		stu3::search::{DateParam, TokenParam},
		Client, FhirStu3, ResourceWrite,
	},
	stu3::{
		codes::{
			AdministrativeGender, EncounterStatus, IssueSeverity, ObservationStatus,
			SearchComparator,
		},
		local_reference_to, reference_to,
		resources::{
			BaseResource, Bundle, Encounter, Observation, ObservationSubjectReferenceTarget,
			OperationOutcome, ParametersParameter, ParametersParameterValue, Patient,
			PatientGeneralPractitionerReferenceTarget, Practitioner, Resource, ResourceType,
			
		},
		types::{CodeableConcept, Coding, HumanName, Identifier, Reference},
	},
	Date,
};
use futures::TryStreamExt;
use uuid::Uuid;

/// Set up a client for testing with the (local) FHIR server.
async fn client() -> Result<Client<FhirStu3>> {
	common::setup_logging().await;
	let base_url =
		env::var("FHIR_SERVER").unwrap_or("http://localhost:8110/fhir/".to_owned()).parse()?;
	Ok(Client::new(base_url)?)
}

/// Go through all entries of the bundle, extracting the outcomes and search for
/// errors inside. Fail if there is any of severity error or fatal.
fn ensure_batch_succeeded(bundle: Bundle) {
	let batch_errors: Vec<&String> = bundle
		.entry
		.iter()
		.flatten()
		.filter_map(|entry| entry.response.as_ref())
		.filter_map(|response| response.outcome.as_ref())
		.filter_map(|resource| <&OperationOutcome>::try_from(resource).ok())
		.flat_map(|outcome| outcome.issue.iter().flatten())
		.filter(|issue| matches!(issue.severity, IssueSeverity::Error | IssueSeverity::Fatal))
		.filter_map(|issue| issue.diagnostics.as_ref())
		.collect();

	assert!(batch_errors.is_empty(), "batch failed, diagnostics {:#?}", batch_errors);
}

#[test]
fn crud() -> Result<()> {
	common::RUNTIME.block_on(crud_inner())
}

async fn crud_inner() -> Result<()> {
	let client = client().await?;

	let mut patient = Patient::builder().active(false).build().unwrap();
	let id = patient.create(&client).await?;
	let resource = client.read::<Patient>(&id).await?.expect("should find resource");
	assert_eq!(resource.active, patient.active);

	patient.active = Some(true);
	patient.update(false, &client).await?;
	patient.active = None;
	patient.update(true, &client).await?;
	let version_id =
		patient.meta.as_ref().and_then(|meta| meta.version_id.as_ref()).expect("get version ID");
	let resource =
		client.read_version::<Patient>(&id, version_id).await?.expect("should find resource");
	assert_eq!(resource.active, patient.active);

	patient.delete(&client).await?;
	let resource = client.read::<Patient>(&id).await?;
	assert_eq!(resource, None);

	Ok(())
}

#[test]
fn read() -> Result<()> {
	common::RUNTIME.block_on(read_inner())
}

async fn read_inner() -> Result<()> {
	let client = client().await?;

	let practitioner: Practitioner =
		Practitioner::builder().id(Uuid::new_v4().to_string()).build().unwrap();
	let practitioner_ref = local_reference_to(&practitioner).unwrap();

	let mut patient = Patient::builder()
		.contained(vec![practitioner.clone().into()])
		.general_practitioner(vec![Some(practitioner_ref.into())])
		.build()
		.unwrap();
	patient.create(&client).await?;

	let read: Patient =
		client.read(patient.id.as_deref().unwrap()).await?.expect("created patient not found");

	// Check the read ID
	assert_eq!(read.id, patient.id);

	let gp_targets: Vec<&PatientGeneralPractitionerReferenceTarget> =
		read.general_practitioner.iter().flatten().filter_map(|r| r.target.as_deref()).collect();

	// Check the Patient.general_practitioner reference target
	assert_eq!(gp_targets, vec![&practitioner.into()]);

	Ok(())
}

#[test]
fn read_referenced() -> Result<()> {
	common::RUNTIME.block_on(read_referenced_inner())
}

async fn read_referenced_inner() -> Result<()> {
	let client = client().await?;

	let mut patient = Patient::builder().build().unwrap();
	patient.create(&client).await?;

	let reference = reference_to(&patient).expect("creating reference");
	let read = client.read_referenced(&reference).await?;
	assert_eq!(read.as_base_resource().id(), patient.id());

	Ok(())
}

#[test]
fn patch_via_fhir() -> Result<()> {
	common::RUNTIME.block_on(patch_via_fhir_inner())
}

async fn patch_via_fhir_inner() -> Result<()> {
	let client = client().await?;

	let mut patient = Patient::builder()
		.active(false)
		.gender(AdministrativeGender::Male)
		.name(vec![Some(HumanName::builder().family("Test".to_owned()).build().unwrap())])
		.build()
		.unwrap();
	patient.create(&client).await?;

	let date = Date::from_str("2021-02-01").expect("parse Date");
	client
		.patch_via_fhir(ResourceType::Patient, patient.id.as_ref().expect("Patient.id"))
		.add(
			"Patient",
			"birthDate",
			ParametersParameter::builder()
				.name("value".to_owned())
				.value(ParametersParameterValue::Date(date.clone()))
				.build()
				.unwrap(),
		)
		.delete("Patient.active")
		.replace(
			"Patient.gender",
			ParametersParameter::builder()
				.name("value".to_owned())
				.value(ParametersParameterValue::Code("female".to_owned()))
				.build()
				.unwrap(),
		)
		.insert(
			"Patient.name",
			ParametersParameter::builder()
				.name("value".to_owned())
				.value(ParametersParameterValue::HumanName(
					HumanName::builder().family("Family".to_owned()).build().unwrap(),
				))
				.build()
				.unwrap(),
			0,
		)
		.send()
		.await?;

	let patient: Patient =
		client.read(patient.id.as_ref().expect("Patient.id")).await?.expect("Patient should exist");
	assert_eq!(patient.birth_date, Some(date));
	assert_eq!(patient.active, None);
	assert_eq!(patient.gender, Some(AdministrativeGender::Female));
	assert_eq!(patient.name.len(), 2);

	Ok(())
}

#[test]
fn patch_via_json() -> Result<()> {
	common::RUNTIME.block_on(patch_via_json_inner())
}

async fn patch_via_json_inner() -> Result<()> {
	let client = client().await?;

	let mut patient = Patient::builder()
		.active(false)
		.gender(AdministrativeGender::Male)
		.name(vec![Some(HumanName::builder().family("Test".to_owned()).build().unwrap())])
		.build()
		.unwrap();
	patient.create(&client).await?;

	let date = Date::from_str("2021-02-01").expect("parse Date");
	client
		.patch_via_json(ResourceType::Patient, patient.id.as_ref().expect("Patient.id"))
		.add("/birthDate", &date)?
		.remove("/active")
		.replace("/gender", AdministrativeGender::Female)?
		.add("/name/0", HumanName::builder().family("Family".to_owned()).build().unwrap())?
		.send()
		.await?;

	let patient: Patient =
		client.read(patient.id.as_ref().expect("Patient.id")).await?.expect("Patient should exist");
	assert_eq!(patient.birth_date, Some(date));
	assert_eq!(patient.active, None);
	assert_eq!(patient.gender, Some(AdministrativeGender::Female));
	assert_eq!(patient.name.len(), 2);

	Ok(())
}

#[test]
fn search() -> Result<()> {
	common::RUNTIME.block_on(search_inner())
}

async fn search_inner() -> Result<()> {
	let client = client().await?;

	let date_str = "5123-05-05";
	let date = Date::from_str(date_str).expect("parse Date");

	let mut patient = Patient::builder().active(false).birth_date(date.clone()).build().unwrap();
	let id = patient.create(&client).await?;

	let patients: Vec<Patient> = client
		.search()
		.with_raw("_id", id)
		.and("birthdate", DateParam { comparator: Some(SearchComparator::Eq), value: date_str })
		.and("active", TokenParam::CodeWithoutSystem { code: "false", not: false })
		.send()
		.await?
		.try_collect()
		.await?;

	assert_eq!(patients.len(), 1);
	assert_eq!(patients[0].active, Some(false));
	assert_eq!(patients[0].birth_date, Some(date));

	patient.delete(&client).await?;
	Ok(())
}

#[test]
fn transaction() -> Result<()> {
	common::RUNTIME.block_on(transaction_inner())
}

async fn transaction_inner() -> Result<()> {
	let client = client().await?;

	let mut patient1 = Patient::builder().build().unwrap();
	patient1.create(&client).await?;
	let mut patient2 = Patient::builder().build().unwrap();
	patient2.create(&client).await?;
	let mut patient3 = Patient::builder().build().unwrap();
	patient3.create(&client).await?;

	let mut transaction = client.transaction();
	transaction.delete(ResourceType::Patient, patient1.id.as_ref().expect("Patient.id"));
	transaction.read(ResourceType::Patient, patient1.id.as_ref().expect("Patient.id"));
	transaction.update(patient3, true)?;
	let patient_ref = transaction.create(Patient::builder().build().unwrap());
	let _encounter_ref = transaction.create(
		Encounter::builder()
			.status(EncounterStatus::Planned)
			.class(
				Coding::builder()
					.system("test-system".to_owned())
					.code("test-code".to_owned())
					.build()
					.unwrap(),
			)
			.subject(Reference::builder().reference(patient_ref.clone()).build().unwrap().into())
			.build()
			.unwrap(),
	);

	let mut entries = transaction.send().await?.0.entry.into_iter().flatten();
	let _delete = entries.next().expect("DELETE response");
	let _read = entries.next().expect("GET response");
	let _update = entries.next().expect("PUT response");
	let _create_patient = entries.next().expect("POST Patient response");
	let create_encounter = entries.next().expect("POST Encounter response");
	assert!(entries.next().is_none());

	let encounter_ref = create_encounter
		.full_url
		.as_ref()
		.or(create_encounter.response.as_ref().and_then(|response| response.location.as_ref()))
		.expect("Encounter ID in response");
	let Resource::Encounter(encounter) = client
		.read_referenced(&Reference::builder().reference(encounter_ref.clone()).build().unwrap())
		.await?
	else {
		panic!("Resource should be Encounter");
	};
	let subject_ref = encounter
		.subject
		.as_ref()
		.expect("Encounter.subject")
		.reference
		.reference
		.as_ref()
		.expect("Encounter.subject.reference");
	println!("Subject reference is: {subject_ref}");
	assert_ne!(subject_ref, &patient_ref);

	Ok(())
}

#[test]
fn unpaged() -> Result<()> {
	common::RUNTIME.block_on(unpaged_inner())
}

async fn unpaged_inner() -> Result<()> {
	let client = client().await?;

	let date = "5123-05-10";
	let n = 99;

	println!("Preparing..");
	let patient = Patient::builder()
		.active(false)
		.birth_date(Date::from_str(date).expect("parse Date"))
		.build()
		.unwrap();
	let mut batch = client.batch();
	for _ in 0..n {
		batch.create(patient.clone());
	}
	ensure_batch_succeeded(batch.send().await?);

	println!("Starting search..");
	let patients: Vec<Patient> = client
		.search()
		.with("birthdate", DateParam { comparator: Some(SearchComparator::Eq), value: date })
		.send()
		.await?
		.try_collect()
		.await?;
	assert_eq!(patients.len(), n);

	println!("Cleaning up..");
	let mut batch = client.batch();
	for patient in patients {
		batch.delete(ResourceType::Patient, patient.id.as_ref().expect("Patient.id"));
	}
	ensure_batch_succeeded(batch.send().await?);
	Ok(())
}

#[test]
fn paged() -> Result<()> {
	common::RUNTIME.block_on(paged_inner())
}

async fn paged_inner() -> Result<()> {
	let client = client().await?;

	let date = "5123-06-10";
	let n = 30;
	let page_size = 20;

	println!("Preparing..");
	let patient = Patient::builder()
		.active(false)
		.birth_date(Date::from_str(date).expect("parse Date"))
		.build()
		.unwrap();
	let mut batch = client.batch();
	for _ in 0..n {
		batch.create(patient.clone());
	}

	let patients = batch.send().await?;
	ensure_batch_succeeded(patients.clone());

	println!("Starting searches..");
	let (page1, next) = client
		.search::<Patient>()
		.with("birthdate", DateParam { comparator: Some(SearchComparator::Eq), value: date })
		.paged(Some(page_size))
		.send()
		.await?;

	let patients1: Vec<Patient> = page1.try_collect().await?;
	assert_eq!(patients1.len() as u32, page_size);

	let (page2, next) = next.expect("second page not found").next_page().await?;
	let patients2: Vec<Patient> = page2.try_collect().await?;

	assert_eq!(patients2.len() as u32, n - page_size);
	assert!(next.is_none());

	println!("Cleaning up..");
	let mut batch = client.batch();
	for patient in patients1.iter().chain(patients2.iter()) {
		batch.delete(ResourceType::Patient, patient.id.as_ref().expect("Patient.id"));
	}
	ensure_batch_succeeded(batch.send().await?);
	Ok(())
}

#[test]
fn paging_with_includes() -> Result<()> {
	common::RUNTIME.block_on(paging_with_includes_inner())
}

async fn paging_with_includes_inner() -> Result<()> {
	let client = client().await?;

	println!("Preparing..");
	let mut batch = client.transaction();

	let birthdate = "5123-05-15";

	let patient = Patient::builder()
		.active(false)
		.birth_date(Date::from_str(birthdate).expect("parse Date"))
		.build()
		.unwrap();

	let patient_uuid = batch.create(patient.clone());

	let observation = Observation::builder()
		.status(ObservationStatus::Final)
		.subject(Reference::builder().reference(patient_uuid).build().unwrap().into())
		.code(CodeableConcept::builder().text("Test observation".to_string()).build().unwrap())
		.build()
		.unwrap();

	// Create enough Observations to fill multiple pages
	let n = 50;

	for _ in 0..n {
		batch.create(observation.clone());
	}

	let resources = batch.send().await?;
	ensure_batch_succeeded(resources.clone());

	println!("Starting search..");
	let observations: Vec<Observation> = client
		.search()
		.with("subject.birthdate", DateParam { comparator: None, value: birthdate })
		.and_raw("_include", "Observation:subject")
		.send()
		.await?
		.try_collect()
		.await?;

	assert_eq!(observations.len(), n);

	let mut patients: Vec<Patient> = client
		.search()
		.with("birthdate", DateParam { comparator: None, value: birthdate })
		.send()
		.await?
		.try_collect()
		.await?;

	if patients.len() > 1 {
		panic!("More than one Patient resource with that birthdate. Unable to run test.")
	}

	let patient = patients.pop().unwrap();

	let expected_subject_target = Box::new(ObservationSubjectReferenceTarget::Patient(patient));

	for observation in observations.iter() {
		assert_eq!(
			observation.subject.as_ref().and_then(|s| s.target.as_ref()),
			Some(&expected_subject_target)
		);
	}

	let mut batch = client.batch();
	for entry in resources.entry.iter().flatten() {
		if let Some(resource) = &entry.resource {
			batch.delete(
				resource.resource_type(),
				resource.as_base_resource().id().as_ref().expect("Resource.id"),
			);
		}
	}
	ensure_batch_succeeded(batch.send().await?);
	Ok(())
}

#[test]
fn operation_encounter_everything() -> Result<()> {
	common::RUNTIME.block_on(operation_encounter_everything_inner())
}

async fn operation_encounter_everything_inner() -> Result<()> {
	let client = client().await?;

	let mut patient = Patient::builder().build().unwrap();
	patient.create(&client).await?;
	let mut encounter = Encounter::builder()
		.status(EncounterStatus::InProgress)
		.class(
			Coding::builder()
				.system("test-system".to_owned())
				.code("test-code".to_owned())
				.build()
				.unwrap(),
		)
		.subject(reference_to(&patient).expect("Patient reference").into())
		.build()
		.unwrap();
	encounter.create(&client).await?;

	let bundle =
		client.operation_encounter_everything(encounter.id.as_ref().expect("Encounter.id")).await?;
	let contains_patient = bundle
		.entry
		.iter()
		.flatten()
		.filter_map(|entry| entry.resource.as_ref())
		.filter_map(|resource| resource.as_base_resource().id().as_ref())
		.any(|id| Some(id) == patient.id.as_ref());
	assert!(contains_patient);

	Ok(())
}

#[test]
#[ignore = "HAPI server does not support this"]
fn operation_patient_match() -> Result<()> {
	common::RUNTIME.block_on(operation_patient_match_inner())
}

async fn operation_patient_match_inner() -> Result<()> {
	let client = client().await?;

	let mut patient = Patient::builder()
		.identifier(vec![Some(Identifier::builder().value("Test".to_owned()).build().unwrap())])
		.build()
		.unwrap();
	patient.create(&client).await?;

	let bundle = client.operation_patient_match(patient.clone(), true, 1).await?;
	let contains_patient = bundle
		.entry
		.iter()
		.flatten()
		.filter_map(|entry| entry.resource.as_ref())
		.filter_map(|resource| resource.as_base_resource().id().as_ref())
		.any(|id| Some(id) == patient.id.as_ref());
	assert!(contains_patient);

	Ok(())
}
