//! Code generation functionality.

mod comments;
mod gen_codes;
mod gen_params;
mod gen_traits;
mod gen_types;

use std::collections::HashMap;

use anyhow::Result;
use inflector::Inflector;
use proc_macro2::{Ident, TokenStream};
use quote::{format_ident, quote};

use crate::model::params::SearchParam;
use crate::model::SearchParamType;
use crate::model::{codes::Code, structures::Type, CodeSystemContentMode, StructureDefinitionKind};

/// Generate the Rust code for the FHIR codes.
pub fn generate_codes(mut codes: Vec<Code>) -> Result<(TokenStream, HashMap<String, String>)> {
	// Collect a map of system URLs to code names to know which codes we have
	// implemented.
	let mut generated_codes = HashMap::new();

	// Deduplicate and sort the codes..
	codes.sort_by_key(|code| code.name.clone());
	codes.dedup_by_key(|code| code.name.clone());

	// Set generation variables.
	let module_doc = " Generated code! Take a look at the generator-crate for changing this file!";

	let codes: Vec<TokenStream> = codes
		.into_iter()
		.filter(|code| {
			!code.name.starts_with(char::is_lowercase)
				&& !code.name.contains(|c: char| c.is_whitespace() || c == '-')
		})
		.filter(|code| code.content == CodeSystemContentMode::Complete)
		.inspect(|code| {
			generated_codes.insert(code.system.clone(), code.name.clone());
		})
		.map(gen_codes::generate_code_enum)
		.collect::<Result<_, _>>()?;

	// Generate the code.
	let gen_enum = quote! {
		#![doc = #module_doc]
		#![allow(clippy::too_many_lines)]

		use std::hash::Hash;
		use serde::{Serialize, Deserialize};
		use super::super::types::{Coding, CodingInner, CodeableConcept, CodeableConceptInner};

		#(#codes)*
	};
	Ok((gen_enum, generated_codes))
}

/// Generate the Rust code for all resource search parameter enums
pub fn generate_search_params(
	search_params: Vec<SearchParam>,
	resources: Vec<Type>,
) -> Result<TokenStream> {
	// Set generation variables.
	let module_doc = " Generated code! Take a look at the generator-crate for changing this file!";

	// Search params of type "special" and params without a FHIRPath expression are
	// currently impossible to generate code for.
	let search_params: Vec<SearchParam> =
		search_params.into_iter().filter(|sp| sp.r#type != SearchParamType::Special).collect();

	let resource_params: Vec<_> = resources
		.into_iter()
		.filter(|r| !r.r#abstract)
		.filter_map(|r| {
			// For now we don't generate variants for the search parameters shared by all
			// resources (i.e. _lastUpdated, _language, etc)
			let res_params: Vec<_> =
				search_params.iter().filter(|sp| sp.base.contains(&r.name)).collect();

			if res_params.is_empty() {
				return None;
			}

			Some((r, res_params))
		})
		.collect();

	let enums = gen_params::generate_search_param_enums(&resource_params);

	let searchable_resource_impls =
		gen_traits::generate_searchable_resource_impls(&resource_params);

	Ok(quote! {
		#![doc = #module_doc]
		#![allow(clippy::too_many_lines)]

		use crate::params::*;
		use super::super::resources;

		#(#enums)*

		#(#searchable_resource_impls)*
	})
}

/// Generate the Rust code for the FHIR types.
pub fn generate_types(
	types: Vec<Type>,
	implemented_codes: &HashMap<String, String>,
) -> Result<TokenStream> {
	// Set generation variables.
	let module_doc = " Generated code! Take a look at the generator-crate for changing this file!";

	let types: Vec<TokenStream> = types
		.iter()
		.filter(|ty| !ty.r#abstract)
		.filter(|ty| ty.kind == StructureDefinitionKind::ComplexType)
		.map(|ty| gen_types::generate_type_struct(ty, implemented_codes))
		.collect::<Result<_, _>>()?;

	// Generate the code.
	Ok(quote! {
		#![doc = #module_doc]
		#![allow(clippy::too_many_lines)]

		use ::core::num::NonZeroU32;
		use std::hash::Hash;
		use serde::{Serialize, Deserialize};
		#[cfg(feature = "builders")]
		use derive_builder::Builder;
		use super::super::*;
		use super::super::codes;
		use super::super::resources::*;
		#[allow(unused_imports)] // Integer64 is unused in R4B.
		use crate::{Base64Binary, Date, DateTime, Decimal, Instant, Time, Integer64};

		#(#types)*

		/// Extension of a field.
		#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
		#[cfg_attr(feature = "builders", derive(Builder))]
		#[serde(rename_all = "camelCase")]
		#[cfg_attr(feature = "builders", builder(
			pattern = "owned",
			name = "FieldExtensionBuilder",
			build_fn(error = "crate::error::BuilderError")
		))]
		pub struct FieldExtension {
			/// Unique id for inter-element referencing
			#[serde(default, skip_serializing_if = "Option::is_none")]
			#[cfg_attr(feature = "builders", builder(default, setter(strip_option)))]
			pub id: Option<String>,
			/// Additional content defined by implementations
			#[serde(default, skip_serializing_if = "Vec::is_empty")]
			#[cfg_attr(feature = "builders", builder(default))]
			pub extension: Vec<Extension>,
		}
		#[cfg(feature = "builders")]
		impl FieldExtension {
			#[doc = "Start building a new FieldExtension."]
			#[must_use]
			pub fn builder() -> FieldExtensionBuilder {
				FieldExtensionBuilder::default()
			}
		}
	})
}

/// Generate the Rust code for the FHIR resources.
pub fn generate_resources(
	resources: Vec<Type>,
	implemented_codes: &HashMap<String, String>,
) -> Result<TokenStream> {
	// Set generation variables.
	let module_doc = " Generated code! Take a look at the generator-crate for changing this file!";

	let mut resource_names = Vec::new();
	let resource_defs: Vec<TokenStream> = resources
		.iter()
		.filter(|ty| !ty.r#abstract)
		.filter(|ty| ty.kind == StructureDefinitionKind::Resource)
		.inspect(|ty| resource_names.push(format_ident!("{}", ty.name)))
		.map(|ty| gen_types::generate_type_struct(ty, implemented_codes))
		.collect::<Result<_, _>>()?;

	let resource_conversions = resource_conversion_impls(&resource_names);
	let resource_type_impls = resource_type_impls(&resource_names);

	let base_resource_impls = gen_traits::generate_base_resource(&resources, implemented_codes)?;
	let domain_resource_impls =
		gen_traits::generate_domain_resource(&resources, implemented_codes)?;
	let named_resource_impls = gen_traits::generate_named_resource(&resources)?;

	// Generate the code.
	Ok(quote! {
		#![doc = #module_doc]
		#![allow(clippy::too_many_lines)]

		use ::core::num::NonZeroU32;
		use std::hash::Hash;
		use serde::{Serialize, Deserialize};
		#[cfg(feature = "builders")]
		use derive_builder::Builder;
		use super::super::*;
		use super::super::codes;
		use super::super::types::*;
		#[allow(unused_imports)] // Integer64 is unused in R4B.
		use crate::{Base64Binary, Date, DateTime, Decimal, Instant, Time, Integer64};
		use crate::error::UnknownResourceType;

		#(#resource_defs)*

		/// Generic resource holding any FHIR resources.
		#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
		#[serde(tag = "resourceType")]
		pub enum Resource {
			#(
				#[doc = stringify!(#resource_names)]
				#resource_names(#resource_names),
			)*
		}

		impl Resource {
			/// Return the resource's type
			#[must_use]
			#[inline]
			pub fn resource_type(&self) -> ResourceType {
				match self {
					#(
						Self::#resource_names(_) => #resource_names::resource_type(),
					)*
				}
			}
		}

		/// Resource type field of the FHIR resources.
		#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
		pub enum ResourceType {
			#(
				#[doc = stringify!(#resource_names)]
				#resource_names,
			)*
		}

		impl ::std::str::FromStr for ResourceType {
			type Err = UnknownResourceType;

			fn from_str(s: &str) -> Result<Self, Self::Err> {
				match s {
					#(
						stringify!(#resource_names) => Ok(ResourceType::#resource_names),
					)*
					_ => Err(UnknownResourceType(s.to_string())),
				}
			}
		}

		impl ::core::fmt::Display for ResourceType {
			fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
				f.write_str(self.as_str())
			}
		}

		/// Wrong resource type for conversion to the specified type.
		#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
		pub struct WrongResourceType;
		impl ::core::fmt::Display for WrongResourceType {
			fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
				write!(f, "The Resource is of a different type than requested")
			}
		}
		impl ::std::error::Error for WrongResourceType {}

		impl From<::core::convert::Infallible> for WrongResourceType {
			fn from(x: ::core::convert::Infallible) -> WrongResourceType {
				match x {}
			}
		}

		#resource_conversions
		#resource_type_impls

		#base_resource_impls
		#domain_resource_impls
		#named_resource_impls
	})
}

/// Conversion implementations between specific resources and the Resource enum.
fn resource_conversion_impls(names: &[Ident]) -> TokenStream {
	quote! {
		#(
			impl From<#names> for Resource {
				fn from(resource: #names) -> Self {
					Self::#names(resource)
				}
			}

			impl TryFrom<Resource> for #names {
				type Error = WrongResourceType;

				fn try_from(resource: Resource) -> Result<Self, Self::Error> {
					match resource {
						Resource::#names(r) => Ok(r),
						_ => Err(WrongResourceType),
					}
				}
			}

			impl<'a> TryFrom<&'a Resource> for &'a #names {
				type Error = WrongResourceType;

				fn try_from(resource: &'a Resource) -> Result<Self, Self::Error> {
					match resource {
						Resource::#names(r) => Ok(r),
						_ => Err(WrongResourceType),
					}
				}
			}

			impl<'a> TryFrom<&'a mut Resource> for &'a mut #names {
				type Error = WrongResourceType;

				fn try_from(resource: &'a mut Resource) -> Result<Self, Self::Error> {
					match resource {
						Resource::#names(r) => Ok(r),
						_ => Err(WrongResourceType),
					}
				}
			}
		)*
	}
}

/// Implementations for the Resource and ResourceType enum.
fn resource_type_impls(names: &[Ident]) -> TokenStream {
	quote! {
		impl ResourceType {
			/// Get the resource type as str.
			#[must_use]
			pub const fn as_str(&self) -> &'static str {
				match self {
					#(
						Self::#names => stringify!(#names),
					)*
				}
			}
		}
	}
}

/// Map field name to proper snake case identifier, with escaped rust keywords.
fn map_field_ident(name: &str) -> Ident {
	match name.to_snake_case().as_str() {
		"type" => format_ident!("r#type"),
		"abstract" => format_ident!("r#abstract"),
		"use" => format_ident!("r#use"),
		"ref" => format_ident!("r#ref"),
		"for" => format_ident!("r#for"),
		"mut" => format_ident!("r#mut"),
		"const" => format_ident!("r#const"),
		name => format_ident!("{name}"),
	}
}

/// Map primitive type to Rust type.
fn map_type(ty: &str) -> Ident {
	match ty {
		"boolean" => format_ident!("bool"),
		"id" | "string" | "code" | "markdown" | "xhtml" => format_ident!("String"),
		"decimal" => format_ident!("Decimal"),
		"unsignedInt" => format_ident!("u32"),
		"positiveInt" => format_ident!("NonZeroU32"),
		"integer" => format_ident!("i32"),
		"uri" | "url" | "oid" | "canonical" => format_ident!("String"),
		"uuid" => format_ident!("String"), // Is a URN, so the `Uuid` type does not fit.
		"base64Binary" => format_ident!("Base64Binary"),
		"date" => format_ident!("Date"),
		"dateTime" => format_ident!("DateTime"),
		"instant" => format_ident!("Instant"),
		"time" => format_ident!("Time"),
		"integer64" => format_ident!("Integer64"), // JSON String, but i64 number
		_ => format_ident!("{ty}"),
	}
}
