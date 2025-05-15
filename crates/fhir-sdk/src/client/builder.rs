//! Builder implementation for the client.

use std::{
	future::Future,
	marker::PhantomData,
	sync::{Arc, Mutex},
};

use reqwest::{header::HeaderValue, Url};

use super::{AuthCallback, Client, Error, RequestSettings};

/// Default user agent of this client.
const DEFAULT_USER_AGENT: HeaderValue =
	HeaderValue::from_static(concat!(env!("CARGO_PKG_NAME"), "/", env!("CARGO_PKG_VERSION")));

/// Builder for the [Client]
pub struct ClientBuilder<Version = super::DefaultVersion> {
	/// The FHIR server's base URL.
	base_url: Option<Url>,
	/// User agent to use for requests.
	/// If a client is set using [`ClientBuilder::client`], this value will
	/// override the user agent value in the [reqwest::Client]
	user_agent: Option<HeaderValue>,
	/// [reqwest::Client] to use for all requests
	client: Option<reqwest_middleware::ClientWithMiddleware>,
	/// Request settings.
	request_settings: Option<RequestSettings>,
	/// Auth callback.
	auth_callback: Option<AuthCallback>,
	/// FHIR version.
	version: PhantomData<Version>,
}

impl<V> Default for ClientBuilder<V> {
	fn default() -> Self {
		Self {
			base_url: None,
			user_agent: None,
			client: None,
			request_settings: None,
			auth_callback: None,
			version: PhantomData,
		}
	}
}

impl<V> ClientBuilder<V> {
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

	/// [reqwest::Client] to use
	#[must_use]
	pub fn client(mut self, client: reqwest::Client) -> Self {
		use reqwest_middleware::ClientBuilder;
		use reqwest_tracing::{SpanBackendWithUrl, TracingMiddleware};

		self.client = Some(
			reqwest_middleware::ClientBuilder::new(client)
				.with(TracingMiddleware::<SpanBackendWithUrl>::new())
				.build(),
		);
		self
	}

	/// Request settings.
	#[must_use]
	pub fn request_settings(mut self, settings: RequestSettings) -> Self {
		self.request_settings = Some(settings);
		self
	}

	/// Set an authorization callback to be called every time the server returns
	/// unauthorized. Should return the header value for the `Authorization`
	/// header.
	#[must_use]
	pub fn auth_callback<F, O, E>(mut self, auth: F) -> Self
	where
		F: Fn() -> O + Send + Sync + Copy + 'static,
		O: Future<Output = Result<HeaderValue, E>> + Send,
		E: Into<Box<dyn std::error::Error + Send + Sync>>,
	{
		self.auth_callback =
			Some(Arc::new(move || Box::pin(async move { (auth)().await.map_err(Into::into) })));
		self
	}

	/// Finalize building the client.
	pub fn build(self) -> Result<Client<V>, Error> {
		let Some(base_url) = self.base_url else {
			return Err(Error::BuilderMissingField("base_url"));
		};
		if base_url.cannot_be_a_base() {
			return Err(Error::UrlCannotBeBase);
		}

		let user_agent = self.user_agent.unwrap_or(DEFAULT_USER_AGENT);

		let client = self.client.unwrap_or_else(|| {
			use reqwest_middleware::ClientBuilder;
			use reqwest_tracing::{SpanBackendWithUrl, TracingMiddleware};

			reqwest_middleware::ClientBuilder::new(reqwest::Client::default())
				.with(TracingMiddleware::<SpanBackendWithUrl>::new())
				.build()
		});

		let request_settings = self
			.request_settings
			.unwrap_or_default()
			.header(reqwest::header::USER_AGENT, user_agent);

		let data = super::ClientData {
			base_url,
			client,
			request_settings: Mutex::new(request_settings),
			auth_callback: Mutex::new(self.auth_callback),
		};
		Ok(Client::from(data))
	}
}

impl<V> Clone for ClientBuilder<V> {
	fn clone(&self) -> Self {
		Self {
			base_url: self.base_url.clone(),
			user_agent: self.user_agent.clone(),
			client: self.client.clone(),
			request_settings: self.request_settings.clone(),
			auth_callback: self.auth_callback.clone(),
			version: self.version,
		}
	}
}

impl<V> std::fmt::Debug for ClientBuilder<V> {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		f.debug_struct("ClientBuilder")
			.field("base_url", &self.base_url)
			.field("user_agent", &self.user_agent)
			.field("client", &self.client)
			.field("request_settings", &self.request_settings)
			.field("auth_callback", &self.auth_callback.as_ref().map(|_| "<fn>"))
			.finish()
	}
}
