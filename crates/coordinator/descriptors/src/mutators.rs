// use core::fmt::Error;
use hdk::prelude::*;
use std::collections::BTreeMap;


use types_descriptor::holon_descriptor::HolonDescriptor;
use types_descriptor::property_descriptor::PropertyDescriptor;
use types_descriptor::type_header::{BaseType,SemanticVersion,TypeHeader};

pub fn create_type_header(
    type_name: String,
    base_type : BaseType,
    description: String,
    is_dependent: bool,
) -> ExternResult<Box<TypeHeader>> {
    let header = Box::new(TypeHeader::new(
        type_name, base_type, description, SemanticVersion::default(), is_dependent));

    Ok(header)
}


/// create_holon_descriptor creates a new HolonDescriptor, but does NOT commit it to storage
pub fn create_holon_descriptor(
    type_name: String,
    description: String,
    is_dependent: bool,
) -> ExternResult<HolonDescriptor> {
    // TODO: Custom Error

    let type_header = create_type_header(
        type_name,
        BaseType::Holon,
        description,
        is_dependent,
    )?;

    let descriptor = HolonDescriptor::new(type_header, BTreeMap::default());

    Ok(descriptor)
}

/*

fn create_property_descriptor(header: Box<TypeHeader>, constraints: Box<PropertyTypeConstraint>)-> ExternResult<Box<PropertyDescriptor>> {
    Ok(Box::new(PropertyDescriptor::new( header, constraints)))
}

pub fn add_property_descriptor(
        parent: ,
        property_name: String,
        base_type: BaseType,
        type_name: String,
        description: String,
        is_dependent: bool,
        constraints: Box<PropertyTypeConstraint>,
    ) -> ExternResult<Self> {
        // TODO: Add guard check that base_type = constraints type

        // Create a TypeHeader
        let header = create_type_header(
            type_name, base_type, description, is_dependent: bool,
        )?;
        // Create a PropertyTypeDescriptor that includes that TypeHeader
        let property_descriptor : Box<PropertyDescriptor> = create_property_descriptor(header, constraints)?;

        // Add the new PropertyTypeDescriptor to the properties B-Tree
        self.properties.insert(property_name, property_descriptor);
        Ok(self.clone())
    }
}
*/