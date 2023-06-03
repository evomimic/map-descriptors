use derive_new::new;
use hdi::prelude::*;
use std::collections::BTreeMap;

use crate::property_descriptor::{CompositeDescriptor, PropertyDescriptor};
use crate::type_header::TypeHeader;


#[hdk_entry_helper]
#[derive(new, Clone, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct HolonDescriptor {
    pub header: Box<TypeHeader>,
    pub properties: CompositeDescriptor,
    // add actions and relationships
}

#[hdk_entry_helper]
#[derive(new, Clone, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct HolonCollectionDescriptor {
    header: Box<TypeHeader>,
    contains_items_of_type: HolonDescriptor,
    min_items: u32,
    max_items: u32,
    unique_items: bool,  // true means duplicate items are not allowed
    is_ordered: bool, // if items have an intrinsic order (e.g., is_ordered=false mathematical set)
}
