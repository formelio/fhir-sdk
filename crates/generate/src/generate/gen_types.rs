//! FHIR types generation.

use std::collections::HashMap;

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
	let ident = format_ident!("{name}");
	let ident_inner = format_ident!("{name}Inner");
	let ident_builder = format_ident!("{name}Builder");
	let ident_builder_str = ident_builder.to_string();

	let mut doc_comment = format!(
		" {} \n\n **[{}]({}) v{}** \n\n {} \n\n {} \n\n ",
		sanitize(&ty.description),
		ty.name,
		ty.url,
		ty.version,
		sanitize(&ty.elements.short),
		sanitize(&ty.elements.definition)
	);
	if let Some(comment) = &ty.elements.comment {
		doc_comment.push_str(&sanitize(comment));
		doc_comment.push(' ');
	}

	let resource_type_field = (ty.kind == StructureDefinitionKind::Resource).then(|| {
		let serde_default = format!("{name}::resource_type");
		let builder_default = format!("ResourceType::{ident}");
		quote! {
			/// Type of this FHIR resource.
			#[doc(hidden)]
			#[serde(default = #serde_default)]
			#[cfg_attr(feature = "builders", builder(default = #builder_default, setter(skip)))]
			resource_type: ResourceType,
		}
	});
	let resource_type_fn = (ty.kind == StructureDefinitionKind::Resource).then_some(quote! {
		impl #ident {
			/// Get the resource type for this FHIR resource.
			#[must_use]
			pub fn resource_type() -> ResourceType {
				ResourceType::#ident
			}
		}
	});

	let (fields, structs): (Vec<_>, Vec<_>) = ty
		.elements
		.fields
		.iter()
		.map(|field| generate_field(field, &ident, ty, implemented_codes))
		.unzip();

	let lookup_references_impl = (ty.kind == StructureDefinitionKind::Resource)
		.then(|| lookup_references_impl(&ident, &ty.elements, true));

	let wrapper_impls = wrapper_impls(&ident, &ident_inner, &ident_builder);

	Ok(quote! {
		#[doc = #doc_comment]
		#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
		#[serde(transparent)]
		pub struct #ident(pub Box<#ident_inner>);

		#[doc = #doc_comment]
		#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
		#[cfg_attr(feature = "builders", derive(Builder))]
		#[serde(rename_all = "camelCase")]
		#[cfg_attr(feature = "builders", builder(
			pattern = "owned",
			name = #ident_builder_str,
			build_fn(error = "crate::error::BuilderError", name = "build_inner"),
		))]
		pub struct #ident_inner {
			#resource_type_field
			#(#fields)*
		}

		#[cfg(feature = "builders")]
		impl #ident_builder {
			#[doc = concat!("Finalize building ", #name, ".")]
			pub fn build(self) -> Result<#ident, crate::error::BuilderError> {
				self.build_inner().map(Into::into)
			}
		}

		#lookup_references_impl
		#wrapper_impls
		#resource_type_fn

		#(#structs)*
	})
}

/// Implement the LookupReferences trait for a type
fn lookup_references_impl(ident: &Ident, field: &ObjectField, is_type: bool) -> TokenStream {
	let refs_pushes: Vec<_> = field
		.fields
		.iter()
		.filter(|f| match f {
			Field::Standard(_) | Field::Code(_) => false,
			Field::Choice(c) => !c.reference_target_resource_types.is_empty(),
			Field::Object(_) | Field::Reference(_) => true,
		})
		.map(|f| {
			let name = f.name().replace("[x]", "");
			let ref_type = format_ident!("{}{}", ident, name.to_pascal_case());
			let name = map_field_ident(&name);
			let name = quote! { #name };
			let path = if is_type {
				quote! { self.0.#name }
			} else {
				quote! { self.#name }
			};

			let is_wrapped = f.optional() || f.is_array();
			let mut push = match f {
				Field::Choice(c) if !c.reference_target_resource_types.is_empty() => {
					if f.optional() {
						quote! {
							if let Some(#ref_type::Reference(ref mut r)) = #path.as_mut() {
								refs.push(Box::new(r));
							}
						}
					} else {
						quote! {
							if let #ref_type::Reference(ref mut r) = &mut #path {
								refs.push(Box::new(r));
							}
						}
					}
				}
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

			// TODO Choices should be fully handled by now, but think of a better way
			if !matches!(f, Field::Choice(_)) {
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
fn wrapper_impls(ident: &Ident, ident_inner: &Ident, ident_builder: &Ident) -> TokenStream {
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

		impl #ident {
			/// Start building an instance.
			#[cfg(feature = "builders")]
			#[must_use]
			pub fn builder() -> #ident_builder {
				#ident_builder ::default()
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
	let builder_attr = field.optional().then_some(
		quote!(#[cfg_attr(feature = "builders", builder(default, setter(strip_option)))]),
	);
	let serde_rename_or_flatten = match field {
		Field::Choice(_) => {
			quote!(#[serde(flatten)])
		}
		Field::Object(_) | Field::Standard(_) | Field::Code(_) | Field::Reference(_) => {
			quote!(#[serde(rename = #name)])
		}
	};

	let extension_field = (!field.is_base_field()).then(|| {
		let ident_ext = format_ident!("{ident}_ext");
		let serde_ext = match field {
			Field::Choice(_) => {
				quote!(#[serde(flatten)])
			}
			Field::Object(_) | Field::Standard(_) | Field::Code(_) | Field::Reference(_) => {
				let rename_ext = format!("_{name}");
				quote!(#[serde(rename = #rename_ext)])
			}
		};

		if field.is_array() {
			quote! {
				/// Extension field.
				#[serde(default, skip_serializing_if = "Vec::is_empty")]
				#serde_ext
				#[cfg_attr(feature = "builders", builder(default))]
				pub #ident_ext: Vec<Option<#extension_type>>,
			}
		} else {
			quote! {
				/// Extension field.
				#[serde(default, skip_serializing_if = "Option::is_none")]
				#serde_ext
				#[cfg_attr(feature = "builders", builder(default, setter(strip_option)))]
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

	let mapped_type = map_type(&field.r#type);

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

	let field_name = field.name.replace("[x]", "");

	let enum_type = format_ident!("{type_ident}{}", field_name.to_pascal_case());
	let enum_doc = format!(" Choice of types for the {} field in {type_ident}", field_name);

	let (reference_struct_ident, reference_types) =
		if !field.reference_target_resource_types.is_empty() {
			let (ident, types) = generate_reference_types(
				&type_ident,
				&field_name,
				&field.reference_target_resource_types,
			);

			(Some(ident), Some(types))
		} else {
			(None, None)
		};

	let variants = field.types.iter().map(|ty| {
		let variant_ident = format_ident!("{}", ty.to_pascal_case());
		let variant_type =
			if ty == "Reference" { reference_struct_ident.clone().unwrap() } else { map_type(ty) };
		let variant_doc = format!(" Variant accepting the {variant_ident} type.");
		let variant_field_name = format!("{field_name}{}", &variant_ident.to_string());

		quote! {
			#[doc = #variant_doc]
			#[serde(rename = #variant_field_name)]
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

	let choice_types = quote! {
		#[doc = #enum_doc]
		#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
		#[serde(rename_all = "camelCase")]
		pub enum #enum_type {
			#(#variants)*
		}

		#[doc = #extension_doc]
		#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
		#[serde(rename_all = "camelCase")]
		pub enum #extension_type {
			#(#extension_variants)*
		}

		#reference_types
	};

	(doc_comment, (quote!(#enum_type), extension_type), choice_types)
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

	let lookup_references_impl = lookup_references_impl(&struct_type, field, false);

	let object_struct_builder = format_ident!("{struct_type}Builder");
	let object_struct_builder_name = object_struct_builder.to_string();
	let object_struct = quote! {
		#[doc = #struct_doc]
		#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
		#[cfg_attr(feature = "builders", derive(Builder))]
		#[serde(rename_all = "camelCase")]
		#[cfg_attr(feature = "builders", builder(pattern = "owned", name = #object_struct_builder_name, build_fn(error = "crate::error::BuilderError")))]
		pub struct #struct_type {
			#(#fields)*
		}

		#[cfg(feature = "builders")]
		impl #struct_type {
			#[doc = "Start building a new instance"]
			#[must_use]
			pub fn builder() -> #object_struct_builder {
				#object_struct_builder ::default()
			}
		}

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

	let struct_type = format_ident!("{type_ident}{}Reference", field.name.to_pascal_case());

	let (_, reference_types) =
		generate_reference_types(&type_ident, &field.name, &field.target_resource_types);

	(doc_comment, (quote!(#struct_type), format_ident!("FieldExtension")), reference_types)
}

pub fn generate_reference_types(
	parent_ident: &Ident,
	parent_field_name: &str,
	target_types: &[String],
) -> (Ident, TokenStream) {
	let struct_type =
		format_ident!("{parent_ident}{}Reference", parent_field_name.to_pascal_case());

	let struct_doc =
		format!(" Reference wrapper type of the {} field in {parent_ident}", parent_field_name);

	// If there are more than one possible target resource types, we define an enum
	// Otherwise, we refer directly to the resource
	let (target_type, target_defs) = if target_types.len() > 1 {
		let target_type = format_ident!(
			"{parent_ident}{}ReferenceTarget",
			parent_field_name.replace("[x]", "").to_pascal_case()
		);

		let enum_doc = format!(
			" Target resources for the {} reference field in {parent_ident}",
			parent_field_name
		);

		let variants = target_types.iter().map(|ty| {
			let variant_type = format_ident!("{}", ty.to_pascal_case());
			let variant_doc = format!(" Variant for {ty} target resources");

			quote! {
				#[doc = #variant_doc]
				#variant_type(#variant_type),
			}
		});

		let match_arms = target_types.iter().map(|resource_type| {
			let rt = format_ident!("{}", resource_type.to_pascal_case());

			quote! {
				Resource::#rt(r) => Ok(#target_type::#rt(r)),
			}
		});

		let from_impls = target_types.iter().map(|resource_type| {
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
			#[derive(Debug, Clone, PartialEq, Eq)]
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

		(Some(target_type), Some(target_defs))
	} else {
		let resource_type = target_types.first().unwrap();
		let target_type = format_ident!("{}", resource_type.to_pascal_case());

		(Some(target_type), None)
	};

	let types = quote! {
		#[doc = #struct_doc]
		#[derive(Debug, Clone, Serialize, Deserialize)]
		pub struct #struct_type {
			#[doc = r" The resource that is being referred to. When doing searches, the client will fill this field if possible."]
			#[serde(skip)]
			pub target: Option<Box<#target_type>>,

			#[doc = r" The FHIR Reference field"]
			#[serde(flatten)]
			pub reference: Reference,
		}

		impl PartialEq for #struct_type {
			fn eq(&self, other: &Self) -> bool {
				self.reference == other.reference
			}
		}

		impl Eq for #struct_type {}

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

	(struct_type, types)
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
		let mapped_type = map_type(&field.r#type);
		quote!(#mapped_type)
	}
}
