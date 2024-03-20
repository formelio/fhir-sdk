//! Generate traits for base resource types.

use std::collections::HashMap;

use anyhow::{anyhow, Result};
use proc_macro2::{Ident, TokenStream};
use quote::{format_ident, quote};

use super::{
	comments::sanitize,
	gen_types::{code_field_type_name, construct_field_type},
	map_field_ident, map_type,
};
use crate::model::{
	structures::{Field, Type},
	StructureDefinitionKind,
};

/// Generate the Element trait definition
pub fn generate_element_def(element: &Type) -> TokenStream {
	let (field_names, field_types) =
		get_field_names_and_types(&element.elements.fields, &HashMap::new());

	let ident = format_ident!("Element");

	make_trait_definition(element, &field_names, &field_types, &ident, &[])
}

/// Generate an impl of the Element trait
pub fn generate_element_impl(ident: &Ident, element: &Type) -> TokenStream {
	let (field_names, field_types) =
		get_field_names_and_types(&element.elements.fields, &HashMap::new());

	let trait_ident = format_ident!("Element");

	make_trait_implementation(ident, &field_names, &field_types, &trait_ident)
}

/// Generate the BaseResource trait and its implementations.
pub fn generate_base_resource(
	resources: &[Type],
	implemented_codes: &HashMap<String, String>,
) -> Result<TokenStream> {
	let resource = resources
		.iter()
		.filter(|ty| ty.r#abstract)
		.filter(|ty| ty.kind == StructureDefinitionKind::Resource)
		.find(|ty| &ty.name == "Resource")
		.ok_or(anyhow!("Could not find base Resource definition"))?;
	let (field_names, field_types) =
		get_field_names_and_types(&resource.elements.fields, implemented_codes);

	let ident = format_ident!("BaseResource");
	let supertraits = [format_ident!("LookupReferences"), format_ident!("TypedResource")];
	let trait_definition =
		make_trait_definition(resource, &field_names, &field_types, &ident, &supertraits);

	let filtered_resource_names: Vec<_> = resources
		.iter()
		.filter(|ty| !ty.r#abstract)
		.filter(|ty| ty.kind == StructureDefinitionKind::Resource)
		.map(|ty| format_ident!("{}", ty.name))
		.collect();

	let trait_implementations: TokenStream = filtered_resource_names
		.iter()
		.map(|name| make_trait_implementation(&name, &field_names, &field_types, &ident))
		.collect();

	let impl_resource_as_trait = quote! {
		impl Resource {
			/// Return the resource as base resource.
			#[must_use]
			#[inline]
			pub fn as_base_resource(&self) -> &dyn #ident {
				match self {
					#(
						Self::#filtered_resource_names(r) => r,
					)*
				}
			}

			/// Return the resource as mutable base resource.
			#[must_use]
			#[inline]
			pub fn as_base_resource_mut(&mut self) -> &mut dyn #ident {
				match self {
					#(
						Self::#filtered_resource_names(r) => r,
					)*
				}
			}
		}
	};

	Ok(quote! {
		#trait_definition
		#trait_implementations
		#impl_resource_as_trait
	})
}

/// Generate the DomainResource trait and its implementations.
pub fn generate_domain_resource(
	resources: &[Type],
	implemented_codes: &HashMap<String, String>,
) -> Result<TokenStream> {
	let resource = resources
		.iter()
		.filter(|ty| ty.r#abstract)
		.filter(|ty| ty.kind == StructureDefinitionKind::Resource)
		.find(|ty| &ty.name == "DomainResource")
		.ok_or(anyhow!("Could not find DomainResource definition"))?;

	// Only generate getters and setters for fields in DomainResource that are not in Resource
	// cause Resource fields are covered in the supertrait BaseResource
	let fields: Vec<_> =
		resource.elements.fields.iter().filter(|ty| !ty.is_base_field()).cloned().collect();

	let (field_names, field_types) = get_field_names_and_types(&fields, implemented_codes);

	let ident = format_ident!("DomainResource");
	let supertraits = [format_ident!("BaseResource")];
	let trait_definition =
		make_trait_definition(resource, &field_names, &field_types, &ident, &supertraits);

	let filtered_resource_names: Vec<_> = resources
		.iter()
		.filter(|ty| !ty.r#abstract)
		.filter(|ty| ty.kind == StructureDefinitionKind::Resource)
		.filter(|ty| ty.base.as_ref().map_or(false, |base| base.ends_with("DomainResource")))
		.map(|ty| format_ident!("{}", ty.name))
		.collect();

	let trait_implementations: TokenStream = filtered_resource_names
		.iter()
		.map(|name| make_trait_implementation(&name, &field_names, &field_types, &ident))
		.collect();

	let impl_resource_as_trait = quote! {
		impl Resource {
			/// Return the resource as domain resource.
			#[must_use]
			#[inline]
			pub fn as_domain_resource(&self) -> Option<&dyn #ident> {
				match self {
					#(
						Self::#filtered_resource_names(r) => Some(r),
					)*
					_ => None,
				}
			}

			/// Return the resource as mutable domain resource.
			#[must_use]
			#[inline]
			pub fn as_domain_resource_mut(&mut self) -> Option<&mut dyn #ident> {
				match self {
					#(
						Self::#filtered_resource_names(r) => Some(r),
					)*
					_ => None,
				}
			}
		}
	};

	Ok(quote! {
		#trait_definition
		#trait_implementations
		#impl_resource_as_trait
	})
}

/// Generate the NamedResource trait and its implementations.
pub fn generate_named_resource(resources: &[Type]) -> Result<TokenStream> {
	let trait_definition = quote! {
		/// Simple trait to supply (const) information about resources.
		pub trait NamedResource {
			/// The FHIR version of this resource.
			const FHIR_VERSION: &'static str;
			/// The ResourceType of this resouce.
			const TYPE: ResourceType;
		}
	};

	let trait_implementations: TokenStream = resources
		.iter()
		.filter(|ty| !ty.r#abstract)
		.filter(|ty| ty.kind == StructureDefinitionKind::Resource)
		.map(|ty| {
			let name = format_ident!("{}", ty.name);
			let version = &ty.version;

			quote! {
				impl NamedResource for #name {
					const FHIR_VERSION: &'static str = #version;
					const TYPE: ResourceType = ResourceType::#name;
				}
			}
		})
		.collect();

	Ok(quote! {
		#trait_definition
		#trait_implementations
	})
}

/// Generate the TypedResource trait and its implementations.
pub fn generate_typed_resource(resources: &[Type]) -> Result<TokenStream> {
	let names: Vec<Ident> = resources
		.iter()
		.filter(|ty| !ty.r#abstract)
		.filter(|ty| ty.kind == StructureDefinitionKind::Resource)
		.map(|ty| format_ident!("{}", ty.name))
		.collect();

	Ok(quote! {
		/// Simple trait to supply the resource type
		pub trait TypedResource {
			/// The ResourceType of this resouce.
			fn resource_type(&self) -> ResourceType;
		}

		#(
			impl TypedResource for #names {
				#[inline]
				fn resource_type(&self) -> ResourceType {
					ResourceType::#names
				}
			}
		)*

		impl TypedResource for Resource {
			fn resource_type(&self) -> ResourceType {
				match self {
					#(
						Self::#names(r) => r.resource_type(),
					)*
				}
			}
		}
	})
}

/// Get field names and types from the elements.
fn get_field_names_and_types(
	fields: &[Field],
	implemented_codes: &HashMap<String, String>,
) -> (Vec<Ident>, Vec<TokenStream>) {
	fields
		.iter()
		.cloned()
		.map(|mut field| {
			field.set_base_field();
			let field_name = map_field_ident(field.name());
			let field_type = match &field {
				Field::Standard(f) => {
					let ty = map_type(&f.r#type);
					quote!(#ty)
				}
				Field::Code(f) => code_field_type_name(f, implemented_codes),
				_ => panic!("Unsupported field type in BaseResource!"),
			};
			(field_name, construct_field_type(&field, field_type))
		})
		.unzip()
}

/// Make a trait definition from a FHIR type, field names and types for the
/// given trait name.
fn make_trait_definition(
	r#type: &Type,
	field_names: &[Ident],
	field_types: &[TokenStream],
	ident: &Ident,
	supertraits: &[Ident],
) -> TokenStream {
	assert_eq!(r#type.name, r#type.elements.name);
	let mut doc_comment = format!(
		" {} \n\n **{} v{}** \n\n {} \n\n {} \n\n ",
		sanitize(&r#type.description),
		r#type.name,
		r#type.version,
		sanitize(&r#type.elements.short),
		sanitize(&r#type.elements.definition),
	);
	if let Some(comment) = &r#type.elements.comment {
		doc_comment.push_str(&sanitize(comment));
		doc_comment.push(' ');
	}

	let mut_getters = field_names.iter().map(|name| format_ident!("{name}_mut"));
	let setters = field_names.iter().map(|name| format_ident!("set_{name}"));

	let name = match supertraits {
		&[] => quote!(#ident),
		traits => quote!(#ident: #(#traits)+*),
	};

	quote! {
		#[doc = #doc_comment]
		pub trait #name {
			#(
				#[doc = concat!("Get `", stringify!(#field_names), "`.")]
				fn #field_names(&self) -> & #field_types;
				#[doc = concat!("Get `", stringify!(#field_names), "` mutably.")]
				fn #mut_getters(&mut self) -> &mut #field_types;
				#[doc = concat!("Set `", stringify!(#field_names), "`.")]
				fn #setters(&mut self, value: #field_types);
			)*
		}
	}
}

/// Make a trait implementation for the resource, given the field names and
/// types for the given trait name.
fn make_trait_implementation(
	name: &Ident,
	field_names: &[Ident],
	field_types: &[TokenStream],
	ident: &Ident,
) -> TokenStream {
	let mut_getters = field_names.iter().map(|name| format_ident!("{name}_mut"));
	let setters = field_names.iter().map(|name| format_ident!("set_{name}"));

	quote! {
		impl #ident for #name {
			#(
				#[inline]
				fn #field_names(&self) -> & #field_types {
					&self.#field_names
				}

				#[inline]
				fn #mut_getters(&mut self) -> &mut #field_types {
					&mut self.#field_names
				}

				#[inline]
				fn #setters(&mut self, value: #field_types) {
					self.#field_names = value;
				}
			)*
		}
	}
}
