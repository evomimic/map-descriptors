use derive_new::new;
use hdi::prelude::*;


use crate::holon_descriptor::HolonDescriptor;
use crate::type_header::TypeHeader;
use crate::value_types::UnitInterval;

#[hdk_entry_helper]
#[derive(new, Clone, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct RelationshipDescriptor {
    pub header: TypeHeader,
    pub source_role: RelationshipRole,
    pub target_role: RelationshipRole,
}

#[hdk_entry_helper]
#[derive(new, Clone, PartialEq,  Eq)]
#[serde(rename_all = "camelCase")]
pub struct RelationshipRole {
    pub role_name: String,
    pub holon_type: HolonDescriptor,
    pub binding_rule: RelationshipBindingRule,
    pub max_multiplicity: u32,
    pub min_multiplicity: u32,
    pub deletion_semantic: DeletionSemantic,
    pub attraction: UnitInterval,

}

#[hdk_entry_helper]
#[derive(new, Clone, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub enum RelationshipBindingRule {
    Auto,    // automatically bind to new version of related holon type
    Manual, // manually decide when to bind to new version of related holon type
}

#[hdk_entry_helper]
#[derive(new, Clone, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub enum DeletionSemantic {
    Block, // prevent deletion of Holon if any Holons are related
    Propagate, // propagate deletion of Holon to related Holons
    Allow, // do nothing with the related Holon
}
