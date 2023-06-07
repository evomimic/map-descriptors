// use core::fmt::Error;
use hdk::prelude::*;

use types_descriptor::holon_descriptor::HolonDescriptor;
use types_descriptor::property_descriptor::{BooleanDescriptor, CompositeDescriptor, IntegerDescriptor, IntegerFormat, PropertyDescriptor, PropertyMap, StringDescriptor};
use types_descriptor::type_header::{BaseType};
use crate::type_header_mutators::new_type_header;

/// new_xxx_descriptor () functions stage new (empty) instances of Descriptors, but do NOT
/// commit them to persistent storage


pub fn new_holon_descriptor(
    type_name: String,
    description: String,
    is_dependent: bool,
) -> ExternResult<HolonDescriptor> {
    let header = new_type_header(type_name, BaseType::Holon, description, is_dependent)?;

    let descriptor = HolonDescriptor::new(header, PropertyMap::new(Default::default()));

    Ok(descriptor)
}


pub fn new_composite_descriptor(
    type_name: String,
    description: String,
    is_dependent: bool,
) -> ExternResult<PropertyDescriptor> {
    let header = new_type_header(type_name, BaseType::Composite, description, is_dependent)?;
    let descriptor = CompositeDescriptor::new(header, PropertyMap::new(Default::default()));
    Ok(PropertyDescriptor::Composite(descriptor))
}



pub fn new_string_descriptor(
    type_name: String,
    description: String,
    is_dependent: bool,
    min_length: u32,
    max_length: u32,
) -> ExternResult<PropertyDescriptor> {
    let header = new_type_header(type_name, BaseType::String, description, is_dependent)?;

    Ok(PropertyDescriptor::String(StringDescriptor::new(
        header, min_length, max_length,
    )))
}

pub fn new_integer_descriptor(
    type_name: &str,
    description: &str,
    is_dependent: bool,
    format: IntegerFormat,
    min_value: i128,
    max_value: i128,
) -> ExternResult<PropertyDescriptor> {
    let header = new_type_header(type_name.to_string(), BaseType::Integer, description.to_string(), is_dependent)?;

    Ok(PropertyDescriptor::Integer(IntegerDescriptor::new(
        header, format, min_value, max_value,
    )))
}

pub fn new_boolean_descriptor(
    type_name: String,
    description: String,
    is_dependent: bool,
    is_fuzzy: bool,
) -> ExternResult<PropertyDescriptor> {
    let header = new_type_header(type_name.to_string(), BaseType::Boolean, description.to_string(), is_dependent)?;

    Ok(PropertyDescriptor::Boolean(BooleanDescriptor::new(
        header, is_fuzzy,
    )))
}
