//! Revision 5 types of FHIR.

pub mod codes;
pub mod resources;
pub mod types;

use self::{
	resources::{Resource, WrongResourceType},
	types::{Reference, ReferenceInner},
};

/// Create relative [`Reference`] to the given resource.
pub fn reference_to(resource: &Resource) -> Option<Reference> {
	Some(
		ReferenceInner {
			reference: Some(format!(
				"{}/{}",
				resource.resource_type(),
				resource.as_base_resource().id.as_ref()?
			)),
			r#type: Some(resource.resource_type().to_string()),
			..Default::default()
		}
		.into(),
	)
}

/// Create local [`Reference`] to the given resource. Make sure the resource is
/// going to be in the `contained` field of the referencing resource.
pub fn local_reference_to(resource: &Resource) -> Option<Reference> {
	Some(
		ReferenceInner {
			reference: Some(format!("#{}", resource.as_base_resource().id.as_ref()?)),
			r#type: Some(resource.resource_type().to_string()),
			..Default::default()
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
