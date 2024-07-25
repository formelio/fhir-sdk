use std::collections::HashSet;
use std::ops::Deref;

use anyhow::Result;
use inflector::Inflector;
use proc_macro2::TokenStream;
use quote::{format_ident, quote};
use regex::Regex;

use crate::model::params::SearchParam;
use crate::model::structures::Type;
use crate::model::SearchParamType;

pub fn generate_search_param_enums(
	resources: Vec<Type>,
	search_params: Vec<SearchParam>,
) -> Vec<TokenStream> {
	let simple_expr_regex = Regex::new(r"^[[:alnum:]\.]+$").unwrap();

	// Search params of type "special" and params without a FHIRPath expression are
	// currently impossible to generate code for.
	let search_params: Vec<SearchParam> = search_params
		.into_iter()
		.filter(|sp| sp.r#type != SearchParamType::Special)
		.filter(|sp| simple_expr_regex.is_match(&sp.expression))
		.collect();

	resources
		.iter()
		// Abstract resources don't need search parameters
		.filter(|r| !r.r#abstract)
		.map(|r| {
			let res_types = ["Resource", r.base.as_deref().unwrap_or_default(), &r.name];

			let res_params: Vec<_> = search_params
				.iter()
				.filter(|sp| sp.base.iter().any(|b| res_types.contains(&b.deref())))
				.collect();

			generate_search_param_enum(r, res_params)
		})
		.collect()
}

pub fn generate_search_param_enum(
	resource: &Type,
	search_params: Vec<&SearchParam>,
) -> TokenStream {
	let name = format_ident!("{}SearchParameter", resource.name.to_pascal_case());

	let arms = search_params
		.iter()
		.map(|sp| format_ident!("{}", sp.name.to_pascal_case()))
		.collect::<Vec<_>>();

	quote! {
		pub enum #name {
			#(#arms,)*
		}
	}
}
