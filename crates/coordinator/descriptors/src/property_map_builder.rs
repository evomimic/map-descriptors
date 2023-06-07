
use types_descriptor::property_descriptor::{PropertyDescriptor, PropertyMap};

///
/// PropertyMapBuilder and its associated methods provide a common way to insert various types
/// of PropertyDescriptors into a BTreeMap that can be leveraged by both Holon and Composite
///

pub fn insert_property_descriptor(property_map: &mut PropertyMap, property_name: String, property_type: PropertyDescriptor) -> Option<PropertyDescriptor> {
    let result = property_map.properties.insert(property_name, property_type);
    result
}

