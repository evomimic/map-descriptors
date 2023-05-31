// use core::fmt::Error;
use hdk::prelude::*;
use std::collections::BTreeMap;

use crate::type_header::create_type_header;

use types_descriptor::{BaseType, HolonDescriptor};

pub fn create_holon_descriptor(
    type_name: String,
    description: String,
    is_dependent: bool,
) -> ExternResult<HolonDescriptor> {
    // TODO: Custom Error

    let type_header = Box::new(create_type_header(type_name, BaseType::Holon, description, is_dependent)?);

    let descriptor = HolonDescriptor::new(type_header, BTreeMap::default());

    Ok(descriptor)
}


