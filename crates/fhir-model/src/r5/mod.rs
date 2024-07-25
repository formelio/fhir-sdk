//! Revision 5 types of FHIR.

pub mod codes;
pub mod params;
pub mod resources;
pub mod types;

use self::{
	resources::{BaseResource, NamedResource, Resource, WrongResourceType},
	types::{Reference, ReferenceInner},
};

/// Create relative [`Reference`] to the given resource.
pub fn reference_to<R>(resource: &R) -> Option<Reference>
where
	R: NamedResource + BaseResource,
{
	Some(
		ReferenceInner {
			id: None,
			extension: Vec::new(),
			reference: Some(format!("{}/{}", R::TYPE, resource.id().as_ref()?)),
			reference_ext: None,
			r#type: Some(R::TYPE.to_string()),
			type_ext: None,
			identifier: None,
			identifier_ext: None,
			display: None,
			display_ext: None,
		}
		.into(),
	)
}

/// Create local [`Reference`] to the given resource. Make sure the resource is
/// going to be in the `contained` field of the referencing resource.
pub fn local_reference_to<R>(resource: &R) -> Option<Reference>
where
	R: NamedResource + BaseResource,
{
	Some(
		ReferenceInner {
			id: None,
			extension: Vec::new(),
			reference: Some(format!("#{}", resource.id().as_ref()?)),
			reference_ext: None,
			r#type: Some(R::TYPE.to_string()),
			type_ext: None,
			identifier: None,
			identifier_ext: None,
			display: None,
			display_ext: None,
		}
		.into(),
	)
}

/// Trait implemented by all FHIR Reference field types
pub trait ReferenceField {
	/// Set the target field
	fn set_target(&mut self, target: Resource) -> Result<(), WrongResourceType>;

	/// Get a borrow to the FHIR Reference field
	fn reference(&self) -> &Reference;

	/// Get a mutable borrow to the FHIR Reference field
	fn reference_mut(&mut self) -> &mut Reference;
}

/// Trait implemented on object types to get mutable borrows to all non-empty reference fields
pub trait LookupReferences {
	/// Get mutable borrows to all the non-empty fields of type Reference in this type
	fn lookup_references(&mut self) -> Vec<Box<&mut dyn ReferenceField>>;
}
