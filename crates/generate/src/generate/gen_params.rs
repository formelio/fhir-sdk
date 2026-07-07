use inflector::Inflector;
use proc_macro2::TokenStream;
use quote::{format_ident, quote};

use crate::model::{params::SearchParam, structures::Type};

pub fn generate_search_param_enums(
	resource_params: &[(Type, Vec<&SearchParam>)],
) -> Vec<TokenStream> {
	resource_params
		.iter()
		.map(|(res, params)| {
			let res_name = &res.name;
			let params_ident = format_ident!("{}SearchParameter", res_name.to_pascal_case());

			let variants = params
				.iter()
				.map(|sp| {
					// Descriptions for search parameters with multiple bases are enormous,
					// so for those cases we only find the line that describes it for the
					// current resource.
					let desc = if sp.description.starts_with("Multiple Resources") {
						let resource_link = format!("[{}]", res.name);

						sp.description
							.lines()
							.find(|l| l.contains(&resource_link))
							.and_then(|l| l.split_once(":"))
							.map(|(_, desc)| desc.trim())
							.unwrap_or(&sp.name)
					} else {
						&sp.description
					};

					let doc_comment = format!(" {}", desc);
					let variant = format_ident!("{}", sp.code.to_pascal_case());

					quote! {
						#[doc = #doc_comment]
						#variant
					}
				})
				.collect::<Vec<_>>();

			let code_arms: Vec<_> = params
				.iter()
				.map(|sp| {
					let code = &sp.code;
					let variant = format_ident!("{}", code.to_pascal_case());

					quote! {
						Self::#variant => #code
					}
				})
				.collect();

			let doc_comment = format!(" Search parameters for the {} resource", res_name);

			quote! {
				#[doc = #doc_comment]
				#[derive(Clone, Debug, PartialEq, Eq, Hash)]
				pub enum #params_ident {
					#(#variants,)*
				}

				impl ResourceSearchParameterDefinition for #params_ident {
					fn resource_type(&self) -> &'static str {
						#res_name
					}

					fn code(&self) -> &'static str {
						match self {
							#(#code_arms,)*
						}
					}
				}
			}
		})
		.collect()
}
