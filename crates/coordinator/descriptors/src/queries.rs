// use std::collections::BTreeMap;

use crate::stub_data_creator::*;
use crate::test_helpers::*;
use hdk::prelude::*;
use types_descriptor::holon_descriptor::HolonDescriptor;

#[hdk_extern]
pub fn get_all_holontypes(_: ()) -> ExternResult<Vec<HolonDescriptor>> {
    // ?TODO: Handle Custom Error conversion to WasmError
    let result = create_dummy_data(())?;
    Ok(result)
}

#[hdk_extern]
pub fn get_entry_by_action_hash(action_hash: ActionHash) -> ExternResult<Entry> {
    get_entry_by_action(action_hash)
}
