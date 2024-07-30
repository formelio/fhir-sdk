use std::marker::PhantomData;
use std::ops::Deref;

use reqwest::Url;
use tower_http::body::Full;

#[derive(Default)]
pub struct FhirResponse<V> {
	pub(super) base_url: Option<Url>,
	pub(super) response: hyper::Response<Full>,
	version: PhantomData<V>,
}

impl<V> FhirResponse<V> {
	pub fn new(base_url: Url, response: hyper::Response<Full>) -> Self {
		FhirResponse { base_url: Some(base_url), response, version: PhantomData::default() }
	}
}

impl<V> Deref for FhirResponse<V> {
	type Target = hyper::Response<Full>;

	fn deref(&self) -> &Self::Target {
		&self.response
	}
}
