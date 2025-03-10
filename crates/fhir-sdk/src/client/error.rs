//! Client errors.

use std::sync::Arc;

#[cfg(feature = "r4b")]
use fhir_model::r4b;
#[cfg(feature = "r5")]
use fhir_model::r5;
#[cfg(feature = "stu3")]
use fhir_model::stu3;
use reqwest::StatusCode;
use thiserror::Error;

/// FHIR REST Client Error.
#[derive(Debug, Clone, Error)]
pub enum Error {
	/// Builder is missing a field to construct the client.
	#[error("Builder is missing field `{0}` to construct the client")]
	BuilderMissingField(&'static str),

	/// URL cannot be a base URL.
	#[error("Given base URL cannot be a base URL")]
	UrlCannotBeBase,

	/// Failed parsing an URL.
	#[error("Failed parsing the URL: {0}")]
	UrlParse(String),

	/// Missing resource ID.
	#[error("Resource is missing ID")]
	MissingId,

	/// Missing resource version ID.
	#[error("Resource is missing version ID")]
	MissingVersionId,

	/// Missing reference field in FHIR reference.
	#[error("Missing reference URL in reference")]
	MissingReference,

	/// Reference was to local resource.
	#[error("Tried to fetch local reference")]
	LocalReference,

	/// Request was not clonable.
	#[error("Was not able to clone HTTP Request")]
	RequestNotClone,

	/// Found URL with unexpected origin.
	#[error("Found URL with unexpected origin: {0}")]
	DifferentOrigin(String),

	/// Auth callback error.
	#[error("Authorization callback error: {0}")]
	AuthCallback(String),

	/// Serialization/Deserialization error.
	#[error("JSON error: {0}")]
	Json(Arc<serde_json::Error>),

	/// HTTP Request error.
	#[error("Request error: {0}")]
	Request(Arc<reqwest::Error>),

	/// HTTP error response.
	#[error("Got error response ({0}): {1}")]
	Response(StatusCode, String),

	#[cfg(feature = "r4b")]
	/// OperationOutcome.
	#[error("OperationOutcome({0}): {1:?}")]
	OperationOutcomeR4B(StatusCode, r4b::resources::OperationOutcome),

	#[cfg(feature = "r5")]
	/// OperationOutcome.
	#[error("OperationOutcome({0}): {1:?}")]
	OperationOutcomeR5(StatusCode, r5::resources::OperationOutcome),

	#[cfg(feature = "stu3")]
	/// OperationOutcome.
	#[error("OperationOutcome({0}): {1:?}")]
	OperationOutcomeStu3(StatusCode, stu3::resources::OperationOutcome),

	/// Resource was not found.
	#[error("Resource `{0}` was not found")]
	ResourceNotFound(String),

	/// Error parsing ETag to version ID, i.e. missing ETag or wrong format.
	#[error("Missing or wrong ETag in response: {0}")]
	EtagFailure(String),

	/// Response did not provide `Location` header or it failed to parse.
	#[error("Missing or wrong Location header in response: {0}")]
	LocationFailure(String),

	/// Wrong resource was delivered.
	#[error("Resource type {0} is not the requested type {1}")]
	WrongResourceType(String, String),

	/// Unexpected resource type.
	#[error("Unexpected resource type {0}")]
	UnexpectedResourceType(String),
}

impl From<serde_json::Error> for Error {
	fn from(error: serde_json::Error) -> Self {
		Self::Json(Arc::new(error))
	}
}

impl From<reqwest::Error> for Error {
	fn from(error: reqwest::Error) -> Self {
		Self::Request(Arc::new(error))
	}
}

impl Error {
	/// Whether the error should likely be retried.
	#[must_use]
	pub fn should_retry(&self) -> bool {
		tracing::debug!("Checking if error `{self}` should be retried");
		match self {
			Self::Request(err) => err.is_connect() || err.is_request() || err.is_timeout(),
			_ => false,
		}
	}
}
