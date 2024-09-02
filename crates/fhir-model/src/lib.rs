//! # FHIR Models.
//!
//! This is a sub-crate of [`fhir-sdk`](https://crates.io/crates/fhir-sdk). Please take a look at the main crate for
//! more documentation.

#![recursion_limit = "1024"]

mod date_time;
mod error;
#[cfg(feature = "r4b")]
pub mod r4b;
#[cfg(feature = "r5")]
pub mod r5;
#[cfg(feature = "stu3")]
pub mod stu3;

use std::ops::{Deref, DerefMut};
use std::str::FromStr;

use base64::prelude::{Engine, BASE64_STANDARD};
use bigdecimal::BigDecimal;
use serde::{Deserialize, Serialize};
use serde_json::value::RawValue;

use crate::error::UnknownResourceType;

pub use self::date_time::*;
pub use bigdecimal;
pub use time;

/// Parsed parts of a FHIR reference. Can be one of local reference, relative
/// reference or absolute reference. The absolute reference is unchecked and can
/// be anything, it is used as fallback.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ParsedReference<'a> {
	/// Local reference, the resource is to be found in the `contained` field.
	Local {
		/// Local ID of the resource.
		id: &'a str,
	},
	/// Relative reference, the resource is on the same FHIR server.
	Relative {
		/// Resource type.
		resource_type: &'a str,
		/// Resource ID.
		id: &'a str,
		/// Targeted version ID if set.
		version_id: Option<&'a str>,
	},
	/// Absolute reference, the resource can be anywhere, might not even be FHIR
	/// server. Might not be a URL, but a URI like a `urn:uuid:...`.
	Absolute {
		/// The FHIR base URL for FHIR server references, or the whole
		/// URL for non-FHIR server URLs or URNs
		base_url: &'a str,
		/// Assumed resource type part if exists. Is just the positional URL
		/// segment, could be wrong.
		resource_type: Option<&'a str>,
		/// Assumed resource ID part if exists. Is just the positional URL
		/// segment, could be wrong.
		id: Option<&'a str>,
		/// Targeted version ID if set.
		version_id: Option<&'a str>,
	},
}

impl<'a> ParsedReference<'a> {
	/// Parse the reference from a reference string.
	#[must_use]
	pub fn new<R: FromStr<Err = UnknownResourceType>>(reference: &'a str) -> Self {
		if reference.starts_with('#') {
			return Self::Local { id: reference.split_at(1).1 };
		}

		let Some((base_url, resource_type, id, version_id)) = Self::parse_segments::<R>(reference)
		else {
			return Self::Absolute {
				base_url: reference,
				resource_type: None,
				id: None,
				version_id: None,
			};
		};

		if base_url.is_some() {
			Self::Absolute {
				base_url: base_url.unwrap(),
				resource_type: Some(resource_type),
				id: Some(id),
				version_id,
			}
		} else {
			Self::Relative { resource_type, id, version_id }
		}
	}

	/// Helper function to split the reference segments if possible.
	/// Returns the base URL (if absolute), the resource type, resource ID and version ID (if versioned).
	/// Returns None if the reference could not be parsed as an absolute or relative reference
	/// to a FHIR resource on a FHIR server.
	///
	/// e.g. "https://fhir.hapi.org/fhir/Patient/123/_history/456" is parsed to
	/// (Some("https://fhir.hapi.org/fhir"),  "Patient", "123", Some("456"))
	fn parse_segments<R: FromStr<Err = UnknownResourceType>>(
		reference: &'a str,
	) -> Option<(Option<&'a str>, &'a str, &'a str, Option<&'a str>)> {
		let mut segments = reference.rsplitn(3, '/');
		let id_or_version = segments.next()?;
		let history_or_type = segments.next()?;
		let base = segments.next();

		let parsed = if history_or_type == "_history" {
			let mut segments = base?.rsplitn(3, '/');
			let id = segments.next()?;
			let resource_type = segments.next()?;
			let base = segments.next();
			(base, resource_type, id, Some(id_or_version))
		} else {
			(base, history_or_type, id_or_version, None)
		};

		// Check if the resource type segment is actually a resource type
		R::from_str(parsed.1).is_ok().then_some(parsed)
	}

	/// Get the resource type that this reference points to as string reference.
	/// Might not be present/visible in absolute URLs or local references. In
	/// absolute URLs, it might also return garbage, as the URL might have
	/// enough segments, but the segment was not an actual resource type. Take
	/// care when parsing to `ResourceType`.
	#[must_use]
	pub fn resource_type(&self) -> Option<&'a str> {
		match self {
			Self::Local { .. } => None,
			Self::Relative { resource_type, .. } => Some(resource_type),
			Self::Absolute { resource_type, .. } => *resource_type,
		}
	}

	/// Get the resource ID that this reference points to. Might not be
	/// present/visible in absolute URLs. In absolute URLs, it might also return
	/// garbage, as the URL might have enough segments, but the segment was not
	/// an actual id. Take care when using it.
	#[must_use]
	pub fn id(&self) -> Option<&'a str> {
		match self {
			Self::Local { id } => Some(id),
			Self::Relative { id, .. } => Some(id),
			Self::Absolute { id, .. } => *id,
		}
	}

	/// Strips off the version segments
	#[must_use]
	pub fn to_versionless(&self) -> Self {
		match *self {
			Self::Local { .. } => *self,
			Self::Relative { resource_type, id, .. } => {
				Self::Relative { resource_type, id, version_id: None }
			}
			Self::Absolute { base_url, resource_type, id, .. } => {
				Self::Absolute { base_url, resource_type, id, version_id: None }
			}
		}
	}

	/// Strips off the base URL, making absolute references relative
	#[must_use]
	pub fn to_unqualified(&self) -> Self {
		match *self {
			Self::Local { .. } | Self::Relative { .. } => *self,
			Self::Absolute {
				resource_type: Some(resource_type), id: Some(id), version_id, ..
			} => Self::Relative { resource_type, id, version_id },
			_ => panic!("Local references can't be made relative"),
		}
	}

	/// Sets a base URL for this reference. Updates the base URL for absolute
	/// references and turns relative references into absolute ones.
	///
	/// Panics for local references
	#[must_use] pub fn with_base_url(self, mut base_url: &'a str) -> Self {
		if base_url.ends_with('/') {
			base_url = base_url.strip_suffix('/').unwrap();
		}

		match self {
			Self::Relative { resource_type, id, version_id } => Self::Absolute {
				base_url,
				resource_type: Some(resource_type),
				id: Some(id),
				version_id,
			},
			Self::Absolute { resource_type, id, version_id, .. } => {
				Self::Absolute { base_url, resource_type, id, version_id }
			}
			Self::Local { .. } => panic!("Local reference can't be made absolute"),
		}
	}
}

impl ToString for ParsedReference<'_> {
	fn to_string(&self) -> String {
		match *self {
			ParsedReference::Local { id } => format!("#{id}"),
			ParsedReference::Relative { resource_type, id, version_id: None } => {
				[resource_type, id].join("/")
			}
			ParsedReference::Relative { resource_type, id, version_id: Some(version_id) } => {
				[resource_type, id, "_history", version_id].join("/")
			}
			ParsedReference::Absolute {
				base_url,
				resource_type: None,
				id: None,
				version_id: None,
			} => base_url.to_string(),
			ParsedReference::Absolute {
				base_url,
				resource_type: Some(resource_type),
				id: Some(id),
				version_id: None,
			} => [base_url, resource_type, id].join("/"),
			ParsedReference::Absolute {
				base_url,
				resource_type: Some(resource_type),
				id: Some(id),
				version_id: Some(version_id),
			} => [base_url, resource_type, id, "_history", version_id].join("/"),
			_ => panic!("Invalid reference"),
		}
	}
}

/// FHIR `integer64` type. Wraps an i64, but serializes and deserializes as
/// string.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct Integer64(pub i64);

impl Serialize for Integer64 {
	fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
	where
		S: serde::Serializer,
	{
		self.0.to_string().serialize(serializer)
	}
}

impl<'de> Deserialize<'de> for Integer64 {
	fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
	where
		D: serde::Deserializer<'de>,
	{
		let s = String::deserialize(deserializer)?;
		let i = s.parse().map_err(serde::de::Error::custom)?;
		Ok(Self(i))
	}
}

/// FHIR `base64Binary` type. Wraps binary data and serializes to base64
/// strings.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Default)]
pub struct Base64Binary(pub Vec<u8>);

impl Serialize for Base64Binary {
	fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
	where
		S: serde::Serializer,
	{
		let s = BASE64_STANDARD.encode(&self.0);
		s.serialize(serializer)
	}
}

impl<'de> Deserialize<'de> for Base64Binary {
	fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
	where
		D: serde::Deserializer<'de>,
	{
		let mut s = String::deserialize(deserializer)?;
		s.retain(|c| !c.is_whitespace());
		let bytes = BASE64_STANDARD.decode(s).map_err(serde::de::Error::custom)?;
		Ok(Self(bytes))
	}
}

/// Wrapper around [BigDecimal] that always serializes
/// the decimal as a JSON number.
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Deserialize, Hash, Default)]
#[serde(transparent)]
pub struct Decimal(BigDecimal);

impl Serialize for Decimal {
	fn serialize<S>(&self, s: S) -> Result<S::Ok, S::Error>
	where
		S: serde::Serializer,
	{
		RawValue::from_string(self.to_string()).unwrap().serialize(s)
	}
}

/// Deref and From implementations for wrappers.
macro_rules! wrapper_impls {
	($wrapper:ident, $inner_type:ty) => {
		impl Deref for $wrapper {
			type Target = $inner_type;

			fn deref(&self) -> &Self::Target {
				&self.0
			}
		}

		impl DerefMut for $wrapper {
			fn deref_mut(&mut self) -> &mut Self::Target {
				&mut self.0
			}
		}

		impl From<$inner_type> for $wrapper {
			fn from(inner: $inner_type) -> Self {
				Self(inner)
			}
		}

		impl From<$wrapper> for $inner_type {
			fn from(wrapper: $wrapper) -> $inner_type {
				wrapper.0
			}
		}
	};
}

wrapper_impls!(Integer64, i64);
wrapper_impls!(Base64Binary, Vec<u8>);
wrapper_impls!(Time, time::Time);
wrapper_impls!(Instant, time::OffsetDateTime);
wrapper_impls!(Decimal, BigDecimal);
