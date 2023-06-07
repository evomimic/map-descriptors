// use std::collections::BTreeMap;

use crate::mutators::{new_holon_descriptor, new_integer_descriptor};
use hdk::prelude::*;
use types_descriptor::holon_descriptor::HolonDescriptor;
use types_descriptor::property_descriptor::IntegerFormat;
use crate::property_map_builder::insert_property_descriptor;


pub fn create_dummy_data(_: ()) -> ExternResult<Vec<HolonDescriptor>>{
    // ?TODO: Handle Custom Error conversion to WasmError
    // TODO: Add calls to create properties on each HolonDescriptor, say, 1 Integer, 1 Boolean, and 1 String


    let mut descriptor1: HolonDescriptor = new_holon_descriptor(
        "holon_type_name1".to_string(),
        "holon_type_description1".to_string(),
        false,
    )?;

    let int_descriptor = new_integer_descriptor(
        "an_integer_property_type",
        "desc for integer_property_type descriptor1, ",
        true,
        IntegerFormat::I32(),
        -10,
        100,
    )?;

    insert_property_descriptor(&mut descriptor1.properties, "prop1".to_string(), int_descriptor);


    let mut descriptor2: HolonDescriptor = new_holon_descriptor(
        "holon_type_name2".to_string(),
        "holon_type_description".to_string(),
        false,
    )?;

    let int_descriptor = new_integer_descriptor(
        "an_integer_property_type",
        "desc for integer_property_type descriptor2, ",
        true,
        IntegerFormat::I32(),
        -20,
        200,
    )?;

    insert_property_descriptor(&mut descriptor2.properties, "prop2.1".to_string(), int_descriptor);


    let mut descriptor3: HolonDescriptor = new_holon_descriptor(
        "holon_type_name3".to_string(),
        "holon_type_description3".to_string(),
        false,
    )?;

    let int_descriptor = new_integer_descriptor(
        "an_integer_property_type",
        "desc for integer_property_type descriptor3, ",
        true,
        IntegerFormat::I32(),
        -20,
        200,
    )?;

    insert_property_descriptor(&mut descriptor3.properties, "prop3.1".to_string(), int_descriptor);


    Ok(vec![descriptor1, descriptor2, descriptor3])
}
