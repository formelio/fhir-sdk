use fhir_model::{r4b, r5, stu3};

use crate::model::params::SearchParam;

impl From<r5::resources::SearchParameter> for SearchParam {
	fn from(param: r5::resources::SearchParameter) -> Self {
		let param = param.0;

		let name = param.name;
		let description = param.description;
		let code = param.code;
		let base = param.base.iter().flatten().map(ToString::to_string).collect();
		let r#type = param.r#type.into();
		let expression = param.expression.expect("SearchParameter.expression");
		let comparators = param.comparator.into_iter().flatten().map(Into::into).collect();

		Self { name, description, code, base, r#type, expression, comparators }
	}
}

impl From<r4b::resources::SearchParameter> for SearchParam {
	fn from(param: r4b::resources::SearchParameter) -> Self {
		let param = param.0;

		let name = param.name;
		let description = param.description;
		let code = param.code;
		let base = param.base.iter().flatten().map(ToString::to_string).collect();
		let r#type = param.r#type.into();
		let expression = param.expression.expect("SearchParameter.expression");
		let comparators = param.comparator.into_iter().flatten().map(Into::into).collect();

		Self { name, description, code, base, r#type, expression, comparators }
	}
}

impl From<stu3::resources::SearchParameter> for SearchParam {
	fn from(param: stu3::resources::SearchParameter) -> Self {
		let param = param.0;

		let name = param.name;
		let description = param.description;
		let code = param.code;
		let base = param.base.iter().flatten().map(ToString::to_string).collect();
		let r#type = param.r#type.into();
		let expression = param.expression.expect("SearchParameter.expression");
		let comparators = param.comparator.into_iter().flatten().map(Into::into).collect();

		Self { name, description, code, base, r#type, expression, comparators }
	}
}

/// Parse a Bundle into SearchParams.
pub fn parse_r5(input: &str) -> Vec<SearchParam> {
	let bundle: r5::resources::Bundle =
		serde_json::from_str(input).expect("Deserializing SearchParameter Bundle");

	bundle
		.0
		.entry
		.into_iter()
		.flatten()
		.map(|entry| entry.resource.expect("Bundle.entry.resource"))
		.filter_map(|resource| match resource {
			r5::resources::Resource::SearchParameter(param) => Some(param),
			_ => None,
		})
		.filter(|sp| sp.expression.as_ref().is_some_and(|e| !e.is_empty()))
		.map(SearchParam::from)
		.collect()
}

/// Parse a Bundle into SearchParams.
pub fn parse_stu3(input: &str) -> Vec<SearchParam> {
	let bundle: stu3::resources::Bundle =
		serde_json::from_str(input).expect("Deserializing SearchParameter Bundle");

	bundle
		.0
		.entry
		.into_iter()
		.flatten()
		.map(|entry| entry.resource.expect("Bundle.entry.resource"))
		.filter_map(|resource| match resource {
			stu3::resources::Resource::SearchParameter(param) => Some(param),
			_ => None,
		})
		.filter(|sp| sp.expression.as_ref().is_some_and(|e| !e.is_empty()))
		.map(SearchParam::from)
		.collect()
}

/// Parse a Bundle into SearchParams.
pub fn parse_r4b(input: &str) -> Vec<SearchParam> {
	let bundle: r4b::resources::Bundle =
		serde_json::from_str(input).expect("Deserializing SearchParameter Bundle");

	bundle
		.0
		.entry
		.into_iter()
		.flatten()
		.map(|entry| entry.resource.expect("Bundle.entry.resource"))
		.filter_map(|resource| match resource {
			r4b::resources::Resource::SearchParameter(param) => Some(param),
			_ => None,
		})
		.filter(|sp| sp.expression.as_ref().is_some_and(|e| !e.is_empty()))
		.map(SearchParam::from)
		.collect()
}
