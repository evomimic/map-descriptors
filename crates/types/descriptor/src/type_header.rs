use derive_new::new;
use hdi::prelude::*;



/// TypeHeaders contain the common attributes that all Type Descriptors share.
///

#[hdk_entry_helper]
#[derive(new, Clone, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct TypeHeader {
    // the shared attributes common to all Type Descriptors
    pub type_name: String,
    pub base_type: BaseType,
    pub description: String,
    pub version: SemanticVersion,
    pub is_dependent: bool, // if true, values of this type cannot existing independent of parent object
    // FUTURE pub is_shared_descriptor: bool, // if true, this descriptor itself is stored as a separately identified object
    // IRI? reference to semantic type?
}

/// BaseTypes constitute the set of primitive types that the MAP natively supports and from which
/// agent-defined types maybe derived.
///
/// Whereas agent-defined types can be added or changed independently of MAP releases,
/// Additions or changes to Base Type definitions require
/// re-compilation and a new MAP core release.
///

#[hdk_entry_helper]
#[derive(Clone, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub enum BaseType {
    Holon,
    Collection,
    Composite,
    Relationship,
    Boolean,
    Integer,
    String,
    Enum,
}

#[hdk_entry_helper]
#[derive(new, Clone, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct SemanticVersion {
    major: u8,
    minor: u8,
    patch: u8,
}

impl Default for SemanticVersion {
    fn default() -> Self {
        SemanticVersion {
            major: 0,
            minor: 0,
            patch: 1,
        }
    }
}

