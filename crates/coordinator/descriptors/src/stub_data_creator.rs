// use std::collections::BTreeMap;

use crate::mutators::new_holon_descriptor;
use hdk::prelude::*;
use types_descriptor::holon_descriptor::HolonDescriptor;


pub fn create_dummy_data(_: ()) -> ExternResult<Vec<HolonDescriptor>>{
    // ?TODO: Handle Custom Error conversion to WasmError
    // TODO: Add calls to create properties on each HolonDescriptor, say, 1 Integer, 1 Boolean, and 1 String
    let descriptor1: HolonDescriptor = new_holon_descriptor(
        "holon_type_name1".to_string(),
        "holon_type_description1".to_string(),
        false,
    )?;

    let descriptor2: HolonDescriptor = new_holon_descriptor(
        "holon_type_name2".to_string(),
        "holon_type_description".to_string(),
        false,
    )?;
    let descriptor3: HolonDescriptor = new_holon_descriptor(
        "holon_type_name3".to_string(),
        "holon_type_description3".to_string(),
        false,
    )?;

    let descriptors_vec = vec![descriptor1, descriptor2, descriptor3];

    Ok(descriptors_vec)
}
