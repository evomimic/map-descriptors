// use std::collections::BTreeMap;

use crate::mutators::{
    new_boolean_descriptor, new_holon_descriptor, new_integer_descriptor, new_string_descriptor,
};
use crate::property_map_builder::insert_property_descriptor;
use hdk::prelude::*;
use types_descriptor::holon_descriptor::HolonDescriptor;
use types_descriptor::property_descriptor::IntegerFormat;

pub fn create_dummy_data(_: ()) -> ExternResult<Vec<HolonDescriptor>> {
    let mut descriptor1: HolonDescriptor = new_holon_descriptor(
        "holon_type_name1".to_string(),
        "holon_type_description1".to_string(),
        false,
    )?;

    let int_descriptor1 = new_integer_descriptor(
        "an_integer_property_type",
        "desc for integer_property_type descriptor1, ",
        true,
        IntegerFormat::I32(),
        -10,
        100,
    )?;

    let string_descriptor1 = new_string_descriptor(
        "a_string_property_type".to_string(),
        "desc for string_property_type descriptor1, ".to_string(),
        true,
        1,
        10,
    )?;

    let bool_descriptor1 = new_boolean_descriptor(
        "a_boolean_property_type".to_string(),
        "desc for boolean_property_type descriptor1, ".to_string(),
        true,
        false,
    )?;

    insert_property_descriptor(
        &mut descriptor1.properties,
        "prop1a_int".to_string(),
        int_descriptor1,
    );

    insert_property_descriptor(
        &mut descriptor1.properties,
        "prop2a_string".to_string(),
        string_descriptor1,
    );

    insert_property_descriptor(
        &mut descriptor1.properties,
        "prop3a_bool".to_string(),
        bool_descriptor1,
    );

    let mut descriptor2: HolonDescriptor = new_holon_descriptor(
        "holon_type_name2".to_string(),
        "holon_type_description".to_string(),
        false,
    )?;

    let int_descriptor2 = new_integer_descriptor(
        "an_integer_property_type",
        "desc for integer_property_type descriptor2, ",
        true,
        IntegerFormat::I32(),
        -20,
        200,
    )?;

    let string_descriptor2 = new_string_descriptor(
        "a_string_property_type".to_string(),
        "desc for string_property_type descriptor1, ".to_string(),
        true,
        2,
        20,
    )?;

    let bool_descriptor2 = new_boolean_descriptor(
        "a_boolean_property_type".to_string(),
        "desc for boolean_property_type descriptor1, ".to_string(),
        true,
        false,
    )?;

    insert_property_descriptor(
        &mut descriptor2.properties,
        "prop1b_int".to_string(),
        int_descriptor2,
    );

    insert_property_descriptor(
        &mut descriptor2.properties,
        "prop2b_string".to_string(),
        string_descriptor2,
    );

    insert_property_descriptor(
        &mut descriptor2.properties,
        "prop3b_bool".to_string(),
        bool_descriptor2,
    );

    let mut descriptor3: HolonDescriptor = new_holon_descriptor(
        "holon_type_name3".to_string(),
        "holon_type_description3".to_string(),
        false,
    )?;

    let int_descriptor3 = new_integer_descriptor(
        "an_integer_property_type",
        "desc for integer_property_type descriptor3, ",
        true,
        IntegerFormat::I32(),
        -30,
        300,
    )?;

    let string_descriptor3 = new_string_descriptor(
        "a_string_property_type".to_string(),
        "desc for string_property_type descriptor1, ".to_string(),
        true,
        3,
        30,
    )?;

    let bool_descriptor3 = new_boolean_descriptor(
        "a_boolean_property_type".to_string(),
        "desc for boolean_property_type descriptor1, ".to_string(),
        true,
        false,
    )?;

    insert_property_descriptor(
        &mut descriptor3.properties,
        "prop1c_int".to_string(),
        int_descriptor3,
    );

    insert_property_descriptor(
        &mut descriptor3.properties,
        "prop2c_string".to_string(),
        string_descriptor3,
    );

    insert_property_descriptor(
        &mut descriptor3.properties,
        "prop3c_bool".to_string(),
        bool_descriptor3,
    );

    Ok(vec![descriptor1, descriptor2, descriptor3])
}
