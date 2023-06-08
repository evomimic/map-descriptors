use hdk::prelude::*;

#[hdk_extern]
pub fn commit_holon_descriptor(descriptor: HolonDescriptor) -> ExternResult<ActionHash> {
    create_entry(EntryTypes::Holon(descriptor))?
}
