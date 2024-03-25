//! FHIR types generation.

use std::collections::HashMap;
use std::ops::Not;

use anyhow::Result;
use inflector::Inflector;
use proc_macro2::{Ident, TokenStream};
use quote::{format_ident, quote};

use super::{comments::sanitize, map_field_ident, map_type};
use crate::model::{
	structures::{ChoiceField, CodeField, Field, ObjectField, ReferenceField, StandardField, Type},
	StructureDefinitionKind,
};

/// Generate struct definition for a FHIR type.
pub fn generate_type_struct(
	ty: &Type,
	implemented_codes: &HashMap<String, String>,
) -> Result<TokenStream> {
	let name = &ty.name;
	let ident = map_type(name, true);
	let ident_inner = format_ident!("{ident}Inner");

	let mut doc_comment = format!(
		" {} \n\n **[{}]({}) v{}** \n\n {} \n\n {} \n\n ",
		sanitize(&ty.description),
		name,
		ty.url,
		ty.version,
		sanitize(&ty.elements.short),
		sanitize(&ty.elements.definition)
	);
	if let Some(comment) = &ty.elements.comment {
		doc_comment.push_str(&sanitize(comment));
		doc_comment.push(' ');
	}

	let (base_field, base_deref_impls) = ty
		.base
		.as_ref()
		.map(|base| generate_base_field(base, &ident_inner))
		.unwrap_or((None, None));

	let (fields, structs): (Vec<_>, Vec<_>) = ty
		.elements
		.fields
		.iter()
		.map(|field| generate_field(field, &ident, ty, implemented_codes))
		.unzip();

	// Impl of the LookupReferences trait
	let lookup_references_impl = (ty.kind == StructureDefinitionKind::Resource)
		.then(|| lookup_references_impl(&ident, &ty.elements, true));

	// Impls to and from the inner struct
	let wrapper_impls = generate_wrapper_impls(&ident, &ident_inner);

	// Generate builder macros and impls for non abstract types
	let (builder_macros, builder_impls) =
		ty.r#abstract.not().then(|| generate_builder_parts(ty, &ident)).unwrap_or((None, None));

	let no_mandatory_fields = ty.elements.fields.iter().any(|f| !f.optional()).not();

	// Types with no mandatory fields can derive Default. This will cause issues if
	// types with required fields ever become base types in the FHIR spec.
	let default_derive = no_mandatory_fields.then_some(quote!(#[derive(SmartDefault)]));

	// Non abstract resources get a resource type field
	let resource_type_field = (ty.kind == StructureDefinitionKind::Resource && !ty.r#abstract)
		.then(|| {
			let default = no_mandatory_fields.then(|| quote!(#[default(ResourceType::#ident)]));

			quote! {
				#[doc(hidden)]
				#default
				pub(crate) resource_type: ResourceType,
			}
		});

	// Non abstract resources also need the serde tag attribute
	let resource_type_macro = (ty.kind == StructureDefinitionKind::Resource && !ty.r#abstract)
		.then_some(quote!(#[serde(tag = "resource_type", rename = #name)]));

	Ok(quote! {
		#[doc = #doc_comment]
		#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
		#[serde(transparent)]
		#default_derive
		pub struct #ident(pub Box<#ident_inner>);

		#[doc = #doc_comment]
		#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
		#[serde(rename_all = "camelCase")]
		#builder_macros
		#resource_type_macro
		#default_derive
		pub struct #ident_inner {
			#resource_type_field

			#base_field

			#(#fields)*
		}

		#lookup_references_impl
		#wrapper_impls
		#builder_impls
		#base_deref_impls

		#(#structs)*
	})
}

fn generate_base_field(
	base: &str,
	ident_inner: &Ident,
) -> (Option<TokenStream>, Option<TokenStream>) {
	// The name of the base field is an abbreviation of the type, to shrink long chains
	let field_name = format_ident!(
		"{}",
		base.chars().filter(|c| c.is_uppercase()).collect::<String>().to_lowercase()
	);
	let field_type = map_type(base, true);

	let doc_comment = format!(" The base {base}");

	let field = quote! {
		#[doc = #doc_comment]
		#[serde(flatten)]
		pub #field_name: #field_type,
	};

	let deref_impls = quote! {
		impl ::core::ops::Deref for #ident_inner {
			type Target = #field_type;

			fn deref(&self) -> &Self::Target {
				&self.#field_name
			}
		}

		impl ::core::ops::DerefMut for #ident_inner {
			fn deref_mut(&mut self) -> &mut Self::Target {
				&mut self.#field_name
			}
		}
	};

	(Some(field), Some(deref_impls))
}

fn generate_builder_parts(ty: &Type, ident: &Ident) -> (Option<TokenStream>, Option<TokenStream>) {
	let ident_str = ident.to_string();
	let ident_builder_str = format!("{ident_str}Builder");
	let ident_builder = format_ident!("{ident_builder_str}");

	let builder_macros = if ty.kind == StructureDefinitionKind::Resource {
		quote! {
			#[cfg_attr(feature = "builders", derive(Builder))]
			#[cfg_attr(feature = "builders", builder(
				pattern = "owned",
				name = #ident_builder_str,
				build_fn(error = "crate::error::BuilderError", name = "build_inner"),
			))]
		}
	} else {
		quote! {
			#[cfg_attr(feature = "builders", derive(Builder))]
			#[cfg_attr(feature = "builders", builder(
				pattern = "owned",
				name = #ident_builder_str,
				build_fn(error = "crate::error::BuilderError"),
			))]
		}
	};

	let inner_wrapper = (ty.kind == StructureDefinitionKind::Resource).then_some(quote! {
		#[cfg(feature = "builders")]
		impl #ident_builder {
			#[doc = concat!("Finalize building ", #ident_str, ".")]
			pub fn build(self) -> Result<#ident, crate::error::BuilderError> {
				self.build_inner().map(Into::into)
			}
		}
	});

	let builder_impls = quote! {
		#inner_wrapper

		#[cfg(feature = "builders")]
		impl #ident {
			/// Start building an instance.
			#[must_use]
			pub fn builder() -> #ident_builder {
				#ident_builder ::default()
			}
		}
	};

	(Some(builder_macros), Some(builder_impls))
}

/// Implement the LookupReferences trait for a type
fn lookup_references_impl(ident: &Ident, field: &ObjectField, is_type: bool) -> TokenStream {
	let refs_pushes: Vec<_> = field
		.fields
		.iter()
		.filter(|f| matches!(f, Field::Reference(_) | Field::Object(_)))
		.map(|f| {
			let name = f.name().replace("[x]", "");
			let name = map_field_ident(&name);
			let name = quote! { #name };
			let path = if is_type {
				quote! { self.0.#name }
			} else {
				quote! { self.#name }
			};

			let is_wrapped = f.optional() || f.is_array();
			let mut push = match f {
				Field::Reference(_) if is_wrapped => quote! {
					refs.push(Box::new(#name));
				},
				Field::Reference(_) => quote! {
					refs.push(Box::new(&mut #path));
				},
				Field::Object(_) if is_wrapped => quote! {
					refs.extend(#name.lookup_references());
				},
				Field::Object(_) => quote! {
					refs.extend(#path.lookup_references());
				},
				_ => panic!("Wrong field type"),
			};

			// Wrap in Option check
			if f.optional() || (f.is_array() && !f.is_base_field()) {
				let var = if f.is_array() { &name } else { &path };

				push = quote! {
					if let Some(#name) = #var.as_mut() {
						#push
					}
				};
			}

			// Unwind Vec types
			if f.is_array() {
				push = quote! {
					for #name in #path.iter_mut() {
						#push
					}
				}
			}

			push
		})
		.collect();

	let body = if refs_pushes.is_empty() {
		quote! { Vec::new() }
	} else {
		quote! {
			let mut refs: Vec<Box<&mut dyn ReferenceField>> = Vec::new();

			#(#refs_pushes)*

			refs
		}
	};

	quote! {
		impl LookupReferences for #ident {
			fn lookup_references(&mut self) -> Vec<Box<&mut dyn ReferenceField>> {
				#body
			}
		}
	}
}

/// Implementations of From, Deref and DerefMut towards the inner type.
fn generate_wrapper_impls(ident: &Ident, ident_inner: &Ident) -> TokenStream {
	quote! {
		impl From<#ident_inner> for #ident {
			fn from(inner: #ident_inner) -> Self {
				Self(Box::new(inner))
			}
		}

		impl ::core::ops::Deref for #ident {
			type Target = #ident_inner;

			fn deref(&self) -> &Self::Target {
				&self.0
			}
		}

		impl ::core::ops::DerefMut for #ident {
			fn deref_mut(&mut self) -> &mut Self::Target {
				&mut self.0
			}
		}
	}
}

/// Generate field information and sub-structs.
fn generate_field(
	field: &Field,
	type_ident: &Ident,
	base_type: &Type,
	implemented_codes: &HashMap<String, String>,
) -> (TokenStream, TokenStream) {
	let (doc_comment, (field_type, extension_type), structs) = match field {
		Field::Standard(f) => generate_standard_field(f),
		Field::Code(f) => generate_code_field(f, implemented_codes),
		Field::Choice(f) => generate_choice_field(f, type_ident),
		Field::Object(f) => generate_object_field(f, type_ident, base_type, implemented_codes),
		Field::Reference(f) => generate_reference_field(f, type_ident),
	};

	let name = field.name().replace("[x]", "");
	let ident = map_field_ident(&name);
	let ty = construct_field_type(field, field_type);

	let serde_attr = field.optional().then(|| {
		if field.is_array() {
			quote!(#[serde(default, skip_serializing_if = "Vec::is_empty")])
		} else {
			quote!(#[serde(default, skip_serializing_if = "Option::is_none")])
		}
	});

	let builder_attr = (!base_type.r#abstract && field.optional()).then_some(
		quote!(#[cfg_attr(feature = "builders", builder(default, setter(strip_option)))]),
	);

	let serde_rename_or_flatten = if matches!(field, Field::Choice(_)) {
		quote!(#[serde(flatten)])
	} else {
		quote!(#[serde(rename = #name)])
	};

	let extension_field = (!field.is_base_field()).then(|| {
		let ident_ext = format_ident!("{ident}_ext");
		let serde_ext = if matches!(field, Field::Choice(_)) {
			quote!(#[serde(flatten)])
		} else {
			let rename_ext = format!("_{name}");
			quote!(#[serde(rename = #rename_ext)])
		};

		let builder_attr_ext = base_type.r#abstract.not().then_some(if field.is_array() {
			quote!(#[cfg_attr(feature = "builders", builder(default))])
		} else {
			quote!(#[cfg_attr(feature = "builders", builder(default, setter(strip_option)))])
		});

		if field.is_array() {
			quote! {
				/// Extension field.
				#[serde(default, skip_serializing_if = "Vec::is_empty")]
				#serde_ext
				#builder_attr_ext
				pub #ident_ext: Vec<Option<#extension_type>>,
			}
		} else {
			quote! {
				/// Extension field.
				#[serde(default, skip_serializing_if = "Option::is_none")]
				#serde_ext
				#builder_attr_ext
				pub #ident_ext: Option<#extension_type>,
			}
		}
	});

	let fields = quote! {
		#[doc = #doc_comment]
		#serde_attr
		#builder_attr
		#serde_rename_or_flatten
		pub #ident: #ty,
		#extension_field
	};
	(fields, structs)
}

/// Generate field information and sub-structs for a standard field.
fn generate_standard_field(field: &StandardField) -> (String, (TokenStream, Ident), TokenStream) {
	let mut doc_comment =
		format!(" **{}** \n\n {} \n\n ", sanitize(&field.short), sanitize(&field.definition));
	if let Some(comment) = &field.comment {
		doc_comment.push_str(&sanitize(comment));
		doc_comment.push(' ');
	}

	let mapped_type = map_type(&field.r#type, false);

	(doc_comment, (quote!(#mapped_type), format_ident!("FieldExtension")), quote!())
}

/// Generate field information and sub-structs for a code field.
fn generate_code_field(
	field: &CodeField,
	implemented_codes: &HashMap<String, String>,
) -> (String, (TokenStream, Ident), TokenStream) {
	let mut doc_comment = format!(
		" **[{}]({}); {}** \n\n {} \n\n ",
		field.code_name.as_deref().unwrap_or_default(),
		field.code_url.as_deref().unwrap_or_default(),
		sanitize(&field.short),
		sanitize(&field.definition)
	);
	if let Some(comment) = &field.comment {
		doc_comment.push_str(&sanitize(comment));
		doc_comment.push(' ');
	}

	let mapped_type = code_field_type_name(field, implemented_codes);

	(doc_comment, (mapped_type, format_ident!("FieldExtension")), quote!())
}

/// Generate field information and sub-structs for a choice field.
fn generate_choice_field(
	field: &ChoiceField,
	type_ident: &Ident,
) -> (String, (TokenStream, Ident), TokenStream) {
	let mut doc_comment =
		format!(" **{}** \n\n {} \n\n ", sanitize(&field.short), sanitize(&field.definition));
	if let Some(comment) = &field.comment {
		doc_comment.push_str(&sanitize(comment));
		doc_comment.push(' ');
	}

	let enum_type = format_ident!("{type_ident}{}", field.name.replace("[x]", "").to_pascal_case());
	let enum_doc = format!(" Choice of types for the {} field in {type_ident}", field.name);

	let variants = field.types.iter().map(|ty| {
		let variant_ident = format_ident!("{}", ty.to_pascal_case());
		let variant_type = map_type(ty, false);
		let variant_doc = format!(" Variant accepting the {variant_ident} type.");
		let rename = field.name.replace("[x]", &variant_ident.to_string());

		quote! {
			#[doc = #variant_doc]
			#[serde(rename = #rename)]
			#variant_ident(#variant_type),
		}
	});

	let extension_type = format_ident!("{enum_type}Extension");
	let extension_doc = format!(" Extension value for {enum_type}.");
	let extension_variants = field.types.iter().map(|ty| {
		let variant_ident = format_ident!("{}", ty.to_pascal_case());
		let variant_doc = format!(" Variant accepting the {variant_ident} extension.");
		let rename = format!("_{}", field.name.replace("[x]", &variant_ident.to_string()));

		quote! {
			#[doc = #variant_doc]
			#[serde(rename = #rename)]
			#variant_ident(FieldExtension),
		}
	});

	let choice_enum = quote! {
		#[doc = #enum_doc]
		#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
		#[serde(rename_all = "camelCase")]
		pub enum #enum_type {
			#(#variants)*
		}

		#[doc = #extension_doc]
		#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
		#[serde(rename_all = "camelCase")]
		pub enum #extension_type {
			#(#extension_variants)*
		}
	};
	(doc_comment, (quote!(#enum_type), extension_type), choice_enum)
}

/// Generate field information and sub-structs for a object field.
fn generate_object_field(
	field: &ObjectField,
	type_ident: &Ident,
	base_type: &Type,
	implemented_codes: &HashMap<String, String>,
) -> (String, (TokenStream, Ident), TokenStream) {
	let mut doc_comment =
		format!(" **{}** \n\n {} \n\n ", sanitize(&field.short), sanitize(&field.definition));
	if let Some(comment) = &field.comment {
		doc_comment.push_str(&sanitize(comment));
		doc_comment.push(' ');
	}

	if let Some(content_reference) = &field.content_reference {
		let field_type_name = content_reference.trim_start_matches('#').to_pascal_case();
		let ty = format_ident!("{field_type_name}");
		return (doc_comment, (quote!(#ty), format_ident!("FieldExtension")), quote!());
	}

	let struct_type = format_ident!("{type_ident}{}", field.name.to_pascal_case());

	let struct_doc = format!(" Sub-fields of the {} field in {type_ident}", field.name);

	let (fields, structs): (Vec<_>, Vec<_>) = field
		.fields
		.iter()
		.map(|f| generate_field(f, &struct_type, base_type, implemented_codes))
		.unzip();

	let (builder_macros, builder_impls) = base_type
		.r#abstract
		.not()
		.then(|| generate_builder_parts(&base_type, &struct_type))
		.unwrap_or((None, None));

	let lookup_references_impl = lookup_references_impl(&struct_type, field, false);

	let object_struct = quote! {
		#[doc = #struct_doc]
		#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
		#[serde(rename_all = "camelCase")]
		#builder_macros
		pub struct #struct_type {
			#(#fields)*
		}

		#builder_impls
		#lookup_references_impl
	};

	let structs = [object_struct]
		.into_iter()
		.chain(structs)
		.reduce(|mut l, r| {
			l.extend(r);
			l
		})
		.expect("Cannot fail");

	(doc_comment, (quote!(#struct_type), format_ident!("FieldExtension")), structs)
}

/// Generate field information and sub-structs for a reference field.
fn generate_reference_field(
	field: &ReferenceField,
	type_ident: &Ident,
) -> (String, (TokenStream, Ident), TokenStream) {
	let mut doc_comment =
		format!(" **{}** \n\n {} \n\n ", sanitize(&field.short), sanitize(&field.definition));
	if let Some(comment) = &field.comment {
		doc_comment.push_str(&sanitize(comment));
		doc_comment.push(' ');
	}

	// If there are more than one possible target resource types, we define an enum
	// Otherwise, we refer directly to the resource
	let (target_type, target_defs) = if field.target_resource_types.len() > 1 {
		let target_type =
			format_ident!("{type_ident}{}ReferenceTarget", field.name.to_pascal_case());

		let enum_doc =
			format!(" Target resources for the {} reference field in {type_ident}", field.name);

		let variants = field.target_resource_types.iter().map(|ty| {
			let variant_type = format_ident!("{}", ty.to_pascal_case());
			let variant_doc = format!(" Variant for {ty} target resources");

			quote! {
				#[doc = #variant_doc]
				#variant_type(#variant_type),
			}
		});

		let match_arms = field.target_resource_types.iter().map(|resource_type| {
			let rt = format_ident!("{}", resource_type.to_pascal_case());

			quote! {
				Resource::#rt(r) => Ok(#target_type::#rt(r)),
			}
		});

		let from_impls = field.target_resource_types.iter().map(|resource_type| {
			let rt = format_ident!("{}", resource_type.to_pascal_case());

			quote! {
				impl From<#rt> for #target_type {
					fn from(resource: #rt) -> #target_type {
						#target_type::#rt(resource)
					}
				}
			}
		});

		let target_defs = quote! {
			#[doc = #enum_doc]
			#[derive(Debug, Clone, PartialEq)]
			pub enum #target_type {
				#(#variants)*
			}

			impl TryFrom<Resource> for #target_type {
				type Error = WrongResourceType;

				fn try_from(resource: Resource) -> Result<Self, Self::Error> {
					match resource {
						#(#match_arms)*
						_ => Err(WrongResourceType),
					}
				}
			}

			#(#from_impls)*
		};

		(target_type, Some(target_defs))
	} else {
		let resource_type = field.target_resource_types.get(0).unwrap();
		let target_type = format_ident!("{}", resource_type.to_pascal_case());

		(target_type, None)
	};

	let struct_type = format_ident!("{type_ident}{}Reference", field.name.to_pascal_case());

	let struct_doc = format!(" Reference wrapper type of the {} field in {type_ident}", field.name);

	let reference_struct = quote! {
		#[doc = #struct_doc]
		#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
		pub struct #struct_type {
			#[doc = r" The resource that is being referred to. When doing searches, the client will fill this field if possible."]
			#[serde(skip)]
			pub target: Option<Box<#target_type>>,

			#[doc = r" The FHIR Reference field"]
			#[serde(flatten)]
			pub reference: Reference,
		}

		impl From<Reference> for #struct_type {
			fn from(reference: Reference) -> Self {
				Self {
					target: None,
					reference,
				}
			}
		}

		impl ReferenceField for #struct_type {
			fn set_target(&mut self, target: Resource) -> Result<(), WrongResourceType> {
				self.target = Some(Box::new(target.try_into()?));

				Ok(())
			}

			fn reference(&self) -> &Reference {
				&self.reference
			}

			fn reference_mut(&mut self) -> &mut Reference {
				&mut self.reference
			}
		}

		#target_defs
	};

	(doc_comment, (quote!(#struct_type), format_ident!("FieldExtension")), reference_struct)
}

/// Construct the type of a field.
pub fn construct_field_type(field: &Field, field_type: TokenStream) -> TokenStream {
	if field.is_array() {
		if field.is_base_field() {
			quote!(Vec<#field_type>)
		} else {
			quote!(Vec<Option<#field_type>>)
		}
	} else if field.optional() {
		quote!(Option<#field_type>)
	} else {
		quote!(#field_type)
	}
}

/// Compute the type name of a CodeField.
pub fn code_field_type_name(
	field: &CodeField,
	implemented_codes: &HashMap<String, String>,
) -> TokenStream {
	let contains_name = field
		.code_name
		.as_ref()
		.map_or(false, |code_name| implemented_codes.values().any(|value| value == code_name));
	let contains_url =
		field.code_url.as_ref().map_or(false, |code_url| implemented_codes.contains_key(code_url));
	if field.r#type.as_str() == "code" && (contains_name || contains_url) {
		let type_name = field
			.code_url
			.as_ref()
			.and_then(|code_url| implemented_codes.get(code_url))
			.or(field.code_name.as_ref())
			.expect("Could not get FHIR code name to generate the field's type");
		let ty = format_ident!("{type_name}");
		quote!(codes::#ty)
	} else {
		let mapped_type = map_type(&field.r#type, false);
		quote!(#mapped_type)
	}
}
