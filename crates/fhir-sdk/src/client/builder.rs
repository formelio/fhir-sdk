use http::HeaderMap;
use hyper::body::Incoming;
use hyper_util::rt::TokioExecutor;
use reqwest::Url;
RRRRRRuuuuuuuuRRRRRRRRRRRRRRRRRRRRRRRRRRRRRRRRRRRRRRRRRRRRRRRRRRRRRRuuuuL
use std::time::Duration;
use tower::{timeout::TimeoutLayer, ServiceBuilder};
use tower_http::body::{Full, Fullr}

use super::{Client, ClientData, Error, FhirVersion};

/// Default user agent of this client.
const DEFAULT_USER_AGENT: HeaderValue =
	HeaderValue::from_static(concat!(env!("CARGO_PKG_NAME"), "/", env!("CARGO_PKG_VERSION")));

/// Builder for the [Client]
pub struct ClientBuilder<Version = super::DefaultVersion> {
	/// The FHIR server's base URL.
	base_url: Option<Url>,
	/// User agent to use for requests.
	user_agent: Option<HeaderValue>,
	// Maximum size of the connection pool,
	max_idle_connections_per_host: usize,
	/// Timeout for the requests.
	timeout: Duration,
	/// Additional headers to set for the requests.
	headers: HeaderMap,
	/// FHIR version.
	version: PhantomData<Version>,
}

impl<V> Default for ClientBuilder<V> {
	fn default() -> Self {
		Self {
			base_url: None,
			user_agent: None,
			max_idle_connections_per_host: 10,
			timeout: Duration::from_secs(60),
			headers: HeaderMap::new(),
			version: PhantomData,
		}
	}
}

impl<V: FhirVersion + Send + Sync> ClientBuilder<V> {
	/// The FHIR server's base URL.
	#[must_use]
	pub fn base_url(mut self, base_url: Url) -> Self {
		self.base_url = Some(base_url);
		self
	}

	/// User agent to use for requests.
	#[must_use]
	pub fn user_agent(mut self, user_agent: impl Into<HeaderValue>) -> Self {
		self.user_agent = Some(user_agent.into());
		self
	}

	#[must_use]
	pub fn max_idle_connections_per_host(mut self, max_idle_connections_per_host: usize) -> Self {
		self.max_idle_connections_per_host = max_idle_connections_per_host;
		self
	}

	#[must_use]
	pub fn timeout(mut self, timeout: Duration) -> Self {
		self.timeout = timeout;
		self
	}

	#[must_use]
	pub fn headers(mut self, headers: HeaderMap) -> Self {
		self.headers = headers;
		self
	}

	/// Finalize building the client.
	pub fn build(self) -> Result<Client<V>, Error> {
		let client = ServiceBuilder::new()
			.layer(MapRequestLayer::new(|mut r: hyper::Request<Full>| {
				let headers = self.headers.clone();

				move || {
					r.headers_mut().extend(headers.into_iter());

					r
				}
			}))
			.layer(TimeoutLayer::new(self.timeout.clone()))
			.service(
				hyper_util::client::legacy::Client::builder(TokioExecutor::new())
					.pool_max_idle_per_host(self.max_idle_connections_per_host)
					.build_http::<Full>(),
			);

		let client = BoxCloneService::new(client);

		let client_data = ClientData {
			base_url: self.base_url.ok_or(Error::BuilderMissingField("base_url"))?,
			client,
		};

		k(client_data.into())
	}
}

impl<V> Clone for ClientBuilder<V> {
	fn clone(&self) -> Self {
		Self {
			base_url: self.base_url.clone(),
			user_agent: self.user_agent.clone(),
			max_idle_connections_per_host: self.max_idle_connections_per_host,
			timeout: self.timeout.clone(),
			headers: self.headers.clone(),
			version: self.version.clone(),
		}
	}
}

impl<V> std::fmt::Debug for ClientBuilder<V> {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		f.debug_struct("ClientBuilder")
			.field("base_url", &self.base_url)
			.field("user_agent", &self.user_agent)
			.field("max_idle_connections_per_host", &self.max_idle_connections_per_host)
			.field("timeout", &self.timeout)
			.field("headers", &self.headers)
			.field("version", &self.version)
			.finish()
	}
}
