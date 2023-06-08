use derive_new::new;
use hdi::prelude::*;

use crate::property_descriptor::PropertyMap;
use crate::type_header::TypeHeader;

#[hdk_entry_helper]
#[derive(new, Clone, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct HolonDescriptor {
    pub header: TypeHeader,
    pub properties: PropertyMap,
    // add actions and relationships
}

#[hdk_entry_helper]
#[derive(new, Clone, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct HolonCollectionDescriptor {
    pub header: TypeHeader,
    pub contains_items_of_type: HolonDescriptor,
    pub min_items: u32,
    pub max_items: u32,
    pub unique_items: bool, // true means duplicate items are not allowed
    pub is_ordered: bool, // if items have an intrinsic order (e.g., is_ordered=false mathematical set)
}
