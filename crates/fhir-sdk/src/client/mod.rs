//! REST Client Implementation.
//!
//! Does only work with one FHIR version at a time!

mod builder;
mod error;
mod misc;
#[cfg(feature = "r4b")]
pub mod r4b;
#[cfg(feature = "r5")]
pub mod r5;
mod request;
mod response;
mod search;
#[cfg(feature = "stu3")]
pub mod stu3;
mod write;

use std::{
	future::Future,
	marker::PhantomData,
	pin::Pin,
	sync::{Arc, Mutex},
};

pub use reqwest::{
	header::{self, HeaderMap, HeaderValue},
	StatusCode, Url,
};

use self::response::FhirResponse;
pub use self::{
	builder::ClientBuilder,
	error::Error,
	request::RequestSettings,
	search::{
		Paged, Search, SearchParameter, SearchParameterOrList, SearchParameters, UnpagedSearch,
		UnpagedSearchExecutor,
	},
	write::ResourceWrite,
};

/// Trait implemented by the FHIR version markers to provide some version specific
/// constants
pub trait FhirVersion {
	/// MIME type used in Accept and Content-Type headers
	const JSON_MIME_TYPE: &'static str;
}

/// FHIR client version to use: FHIR STU3.
#[derive(Debug)]
pub struct FhirStu3;

impl FhirVersion for FhirStu3 {
	const JSON_MIME_TYPE: &'static str = "application/fhir+json; fhirVersion=3.0";
}

/// FHIR client version to use: FHIR R4B.
#[derive(Debug)]
pub struct FhirR4B;

impl FhirVersion for FhirR4B {
	const JSON_MIME_TYPE: &'static str = "application/fhir+json; fhirVersion=4.3";
}

/// FHIR client version to use: FHIR R5.
#[derive(Debug)]
pub struct FhirR5;

impl FhirVersion for FhirR5 {
	const JSON_MIME_TYPE: &'static str = "application/fhir+json; fhirVersion=5.0";
}

#[cfg(feature = "r5")]
/// Default client version.
type DefaultVersion = FhirR5;
#[cfg(all(not(feature = "r5"), feature = "r4b"))]
/// Default client version.
type DefaultVersion = FhirR4B;
#[cfg(all(not(feature = "r5"), not(feature = "r4b"), feature = "stu3"))]
/// Default client version.
type DefaultVersion = FhirStu3;

/// FHIR REST Client.
#[derive(Debug)]
pub struct Client<Version = DefaultVersion>(Arc<ClientData>, PhantomData<Version>);

/// Return type of the auth callback.
type AuthCallbackReturn = Result<HeaderValue, Box<dyn std::error::Error + Send + Sync>>;
/// Auth callback function type.
type AuthCallback =
	Arc<dyn Fn() -> Pin<Box<dyn Future<Output = AuthCallbackReturn> + Send>> + Send + Sync>;

/// FHIR Rest Client data.
struct ClientData {
	/// The FHIR server's base URL.
	base_url: Url,
	/// HTTP request client.
	client: reqwest::Client,
	/// Request settings.
	request_settings: Mutex<RequestSettings>,
	/// Authorization callback method, returning the authorization header value.
	auth_callback: Mutex<Option<AuthCallback>>,
}

impl<V> From<ClientData> for Client<V> {
	fn from(data: ClientData) -> Self {
		Self(Arc::new(data), PhantomData)
	}
}

impl<V: FhirVersion + Send + Sync> Client<V> {
	/// Start building a new client with custom settings.
	#[must_use]
	pub fn builder() -> ClientBuilder<V> {
		ClientBuilder::default()
	}

	/// Create a new client with default settings.
	pub fn new(base_url: Url) -> Result<Self, Error> {
		let client = Self::builder().base_url(base_url).build()?;
		Ok(client.convert_version())
	}

	/// Get the request settings configured in this client.
	#[must_use]
	pub fn request_settings(&self) -> RequestSettings {
		#[allow(clippy::expect_used)] // only happens on panics, so we can panic again.
		self.0.request_settings.lock().expect("mutex poisened").clone()
	}

	/// Set the request settings for this client.
	pub fn set_request_settings(&self, settings: RequestSettings) {
		tracing::debug!("Setting new request settings");
		#[allow(clippy::expect_used)] // only happens on panics, so we can panic again.
		let mut request_settings = self.0.request_settings.lock().expect("mutex poisened");
		*request_settings = settings;
	}

	/// Get the auth callback configured in this client.
	#[must_use]
	pub fn auth_callback(&self) -> Option<AuthCallback> {
		#[allow(clippy::expect_used)] // only happens on panics, so we can panic again.
		self.0.auth_callback.lock().expect("mutex poisened").clone()
	}

	/// Convert to a different version.
	fn convert_version<Version>(self) -> Client<Version> {
		Client(self.0, PhantomData)
	}

	/// Switch the client to STU3 mode.
	#[must_use]
	pub fn stu3(self) -> Client<FhirStu3> {
		self.convert_version()
	}

	/// Switch the client to R4B mode.
	#[must_use]
	pub fn r4b(self) -> Client<FhirR4B> {
		self.convert_version()
	}

	/// Switch the client to R5 mode.
	#[must_use]
	pub fn r5(self) -> Client<FhirR5> {
		self.convert_version()
	}

	/// Run a request using the internal request settings, calling the auth
	/// callback to retrieve a new Authorization header on `unauthtorized`
	/// responses.
	async fn run_request(
		&self,
		request: reqwest::RequestBuilder,
	) -> Result<FhirResponse<V>, Error> {
		// Try running the request
		let mut request_settings = self.request_settings();
		let response = request_settings
			.make_request(request.try_clone().ok_or(Error::RequestNotClone)?)
			.await?;

		let base_url = self.0.base_url.clone();

		// On authorization failure, retry after refreshing the authorization header.
		if response.status() == StatusCode::UNAUTHORIZED {
			if let Some(auth_callback) = self.auth_callback() {
				tracing::info!("Hit unauthorized response, calling auth_callback");
				let auth_value = (auth_callback)()
					.await
					.map_err(|err| Error::AuthCallback(format!("{err:#}")))?;
				request_settings = request_settings.header(header::AUTHORIZATION, auth_value);
				self.set_request_settings(request_settings.clone());
				let response = request_settings.make_request(request).await?;

				return Ok(FhirResponse::new(base_url, response));
			}
		}

		Ok(FhirResponse::new(base_url, response))
	}

	/// Send a HTTP GET to a generic URL. `url` has to be on the same origin as this
	/// client's base URL.
	async fn fetch_url(&self, url: Url) -> Result<FhirResponse<V>, Error> {
		// Make sure we are fetching from the same origin
		if url.origin() != self.0.base_url.origin() {
			return Err(Error::DifferentOrigin(url.to_string()));
		}

		let request = self.0.client.get(url).header(header::ACCEPT, V::JSON_MIME_TYPE);

		self.run_request(request).await
	}

	/// Get the URL with the configured base URL and the given path segments.
	fn url(&self, segments: &[&str]) -> Url {
		let mut url = self.0.base_url.clone();
		#[allow(clippy::expect_used)] // We made sure of it in the constructor.
		url.path_segments_mut().expect("Base URL cannot be base").pop_if_empty().extend(segments);
		url
	}
}

impl<V> Clone for Client<V> {
	fn clone(&self) -> Self {
		Self(self.0.clone(), self.1)
	}
}

impl std::fmt::Debug for ClientData {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		let auth_callback = match self.auth_callback.try_lock() {
			Ok(inside) => {
				if inside.is_some() {
					"Some(<fn>)"
				} else {
					"None"
				}
			}
			Err(_) => "<blocked>",
		};

		f.debug_struct("ClientData")
			.field("base_url", &self.base_url)
			.field("client", &self.client)
			.field("request_settings", &self.request_settings)
			.field("auth_callback", &auth_callback)
			.finish()
	}
}
