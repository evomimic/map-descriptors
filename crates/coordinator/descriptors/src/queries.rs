// use std::collections::BTreeMap;

use crate::mutators::create_holon_descriptor;
use hdk::prelude::*;
use types_descriptor::descriptor::HolonDescriptor;

#[hdk_extern]
pub fn get_all_holontypes(_: ()) -> ExternResult<Vec<HolonDescriptor>> {
    // ?TODO: Handle Custom Error conversion to WasmError

    let descriptor1: HolonDescriptor = create_holon_descriptor(
        "holon_type_name1".to_string(),
        "holon_type_description1".to_string(),
        false,
    )?;
    let descriptor2: HolonDescriptor = create_holon_descriptor(
        "holon_type_name2".to_string(),
        "holon_type_description".to_string(),
        false,
    )?;
    let descriptor3: HolonDescriptor = create_holon_descriptor(
        "holon_type_name3".to_string(),
        "holon_type_description3".to_string(),
        false,
    )?;

    let descriptors_vec = vec![descriptor1, descriptor2, descriptor3];

    Ok(descriptors_vec)
}
