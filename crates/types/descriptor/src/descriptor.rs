use hdi::prelude::*;
use derive_new::new;
use std::collections::BTreeMap;

pub type DescriptorId = ActionHash; // the

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
        SemanticVersion { major: 0, minor: 0, patch: 1 }
    }
}

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
#[derive(new,Clone, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct TypeHeader { // the shared attributes common to all Type Descriptors
    pub type_name: String,
    pub base_type: BaseType,
    pub description: String,
    pub version: SemanticVersion,
    pub is_dependent: bool, // if true, values of this type cannot existing independent of parent object
    // FUTURE: pub is_shared_descriptor: bool, // if true, this descriptor itself is stored as a separately identified object
    // IRI? reference to semantic type?
}

#[hdk_entry_helper]
#[derive(new,Clone, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct HolonDescriptor {
    pub header: Box<TypeHeader>,
    pub properties: BTreeMap<String, DependentTypeDescriptor>,
    // add actions and relationships
}

// impl HolonDescriptor {
//     pub fn add_string_property(&self, name: String, min_length: u32, max_length: u32, string_descriptor_id: DescriptorId) -> Self {
        
//         let property_descriptor = DependentTypeDescriptor::new_string(StringDescriptor::new( min_length, max_length))
//         self.properties.insert(name, property_descriptor);
//     }
// }


#[hdk_entry_helper]
#[derive(new,Clone, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct BooleanDescriptor {
    header: TypeHeader,
    is_fuzzy: bool, // if true, this property has FuzzyBoolean value, otherwise just true or false
}
/*
    pub struct CollectionDescriptor {
        header: Box<TypeHeader>,
        contains_items_of_type: HolonDescriptor,
        min_items: u32,
        max_items: u32,
        unique_items: bool,
        // true means duplicate items are not allowed
        is_ordered: bool, // if items have an intrinsic order (e.g., is_ordered=false mathematical set)
    }

    pub struct CompositeDescriptor {
        header: Box<TypeHeader>,
    //    properties: BtreeMap<String, DependentTypeDescriptor>,
    }
*/

/*
    The following enum specifies the subset TypeDescriptors whose instances cannot exist independent
    of a parent instance.

    Dependent types don't have unique identifiers
*/
#[hdk_entry_helper]
#[derive(new, Clone, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub enum DependentTypeDescriptor {
    //Composite(CompositeDescriptor),
    //Collection(CollectionDescriptor),
    // but only for collections of Dependent Types
    Boolean(BooleanDescriptor),
    Integer(IntegerDescriptor),
    String(StringDescriptor),
    //Enum(EnumDescriptor),
}

#[hdk_entry_helper]
#[derive(new,Clone, PartialEq, Eq)]
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
#[derive(new,Clone, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct StringDescriptor {
    header: TypeHeader,
    min_length: u32,
    max_length: u32,
    //pattern: String,
}
/*
    pub struct RelationshipDescriptor {
        header: Box<TypeHeader>,
        source_role: RelationshipRole,
        target_role: RelationshipRole,
    }

    pub struct RelationshipRole {
        role_name: String,
        holon_type: HolonDescriptor,
        binding_rule: RelationshipBindingRule,
        max_multiplicity: u32,
        min_multiplicity: u32,
        deletion_semantic: DeletionSemantic,
        attraction: UnitInterval,

    }

    pub enum RelationshipBindingRule {
        Auto,
        // automatically bind to new version of related holon type
        Manual, // manually decide when to bind to new version of related holon type
    }

    pub enum DeletionSemantic {
        Block,
        // prevent deletion of Holon if any Holons are related
        Propagate,
        // propagate deletion of Holon to related Holons
        Allow, // do nothing with the related Holon
    }

    pub struct UnitInterval {
        value: f32, // value can range from 0 to 1, inclusive
    }

    struct FuzzyBoolean {
        value: UnitInterval, // zero = false, one = true
    }

*/
