use hdk::prelude::*;

pub fn get_entry_by_action(action_hash: ActionHash) -> ExternResult<Entry> {
    let record = get_record_by_action(action_hash, GetOptions::default())?;
    match record.entry() {
        record::RecordEntry::Present(entry) => Ok(entry.clone()),
        _ => Err(wasm_error!("Record {:?} does not have an entry", record)),
    }
}

//   #[allow(clippy::needless_pass_by_value)]
fn get_record_by_action(action_hash: ActionHash, get_options: GetOptions) -> ExternResult<Record> {
    match get(action_hash.clone(), get_options)? {
        Some(record) => Ok(record),
        None => Err(wasm_error!(
            "There is no record at the hash {}",
            action_hash
        )),
    }
}
