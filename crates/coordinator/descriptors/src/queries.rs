// use std::collections::BTreeMap;

use crate::stub_data_creator::*;
use hdk::prelude::*;
use types_descriptor::holon_descriptor::HolonDescriptor;


#[hdk_extern]
pub fn get_all_holontypes(_: ()) -> ExternResult<Vec<HolonDescriptor>> {
    // ?TODO: Handle Custom Error conversion to WasmError
    let result = create_dummy_data(())?;
    Ok(result)
}
