use derive_new::new;
use hdi::prelude::*;
use std::collections::BTreeMap;

use crate::type_header::TypeHeader;
use crate::value_types::UnitInterval;

/// PropertyDescriptor enumerates the subset of TypeDescriptors whose instances cannot exist
/// independent of a parent instance. In other words, they cannot be identified or stored
/// independently of their parent instance

#[hdk_entry_helper]
#[derive(new, Clone, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub enum PropertyDescriptor {
    Boolean(BooleanDescriptor),
    Composite(CompositeDescriptor),
    //Enum(EnumDescriptor),
    Integer(IntegerDescriptor),
    String(StringDescriptor),
    ValueCollection(ValueCollectionDescriptor), // can only contain collections of PropertyTypes (not Holons)
}

#[hdk_entry_helper]
#[derive(new, Clone, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct BooleanDescriptor {
    header: TypeHeader,
    is_fuzzy: bool, // if true, this property has FuzzyBoolean value, otherwise just true or false
}

#[hdk_entry_helper]
#[derive(new, Clone, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct CompositeDescriptor {
    header: Box<TypeHeader>,
    properties: BTreeMap<String, PropertyDescriptor>,
}

#[hdk_entry_helper]
#[derive(new, Clone, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct IntegerDescriptor {
    header: TypeHeader,
    format: IntegerFormat,
    min_value: u128,
    max_value: u128,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub enum IntegerFormat {
    I8(),
    I16(),
    I32(),
    I64(),
    I128(),
    U8(),
    U16(),
    U32(),
    U64(),
    U128(),
}

#[hdk_entry_helper]
#[derive(new, Clone, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct StringDescriptor {
    header: TypeHeader,
    min_length: u32,
    max_length: u32,
    //pattern: String,
}

// This is just a first cut at ValueCollectionDescriptor
// It identifies the kinds of items the collection contains
#[hdk_entry_helper]
#[derive(new, Clone, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct ValueCollectionDescriptor {
    header: Box<TypeHeader>,
    contains_items_of_type: String, // TODO: replace this with a ref
    min_items: u32,
    max_items: u32,
    unique_items: bool, // true means duplicate items are not allowed
    is_ordered: bool, // if items have an intrinsic order
}




