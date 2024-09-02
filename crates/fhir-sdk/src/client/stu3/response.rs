use fhir_model::stu3::resources::{OperationOutcome, Resource, };
use reqwest::Url;

use crate::client::response::FhirResponse;
use crate::client::{Error, FhirStu3};

use super::references::populate_reference_targets;

impl FhirResponse<FhirStu3> {
	/// Attempts to parse the response body as a FHIR resource.
	/// Returns an [Error] if the request failed or the body could not be
	/// parsed as `R`.
	pub async fn body<R: TryFrom<Resource>>(self) -> Result<R, Error> {
		let status = self.status();
		let body = self.response.text().await?;

		if status.is_success() {
			parse(&self.base_url, &body)
		} else if let Ok(outcome) = parse::<OperationOutcome>(&self.base_url, &body) {
			Err(Error::OperationOutcomeStu3(status, outcome))
		} else {
			Err(Error::Response(status, body))
		}
	}

	/// Check whether the request was successful and ignores the body if it is.
	/// If not, attempts to parse the body as an [OperationOutcome] and returns
	/// an [Error]
	pub async fn successful(self) -> Result<(), Error> {
		let status = self.response.status();

		if status.is_success() {
			return Ok(());
		}

		let body = &self.response.text().await.unwrap_or_default();
		if let Ok(outcome) = parse::<OperationOutcome>(&self.base_url, body) {
			Err(Error::OperationOutcomeStu3(status, outcome))
		} else {
			Err(Error::Response(status, body.to_string()))
		}
	}
}

fn parse<R: TryFrom<Resource>>(base_url: &Url, body: &str) -> Result<R, Error> {
	let mut resource: Resource = serde_json::from_str(body)?;
	let resource_type = resource.resource_type();

	populate_reference_targets(base_url, &mut resource);

	resource.try_into().map_err(|_| Error::UnexpectedResourceType(resource_type.to_string()))
}
