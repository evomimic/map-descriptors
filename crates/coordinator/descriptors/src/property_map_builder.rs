/// The PropertyMapBuilder provides a common way to build properties BTreeMaps that can be
/// Shared by both HolonDescriptor and CompositeDescriptor
use hdk::prelude::*;
use std::collections::BTreeMap;

use crate::mutators::{new_boolean_descriptor, new_integer_descriptor, new_string_descriptor, new_type_header};
use types_descriptor::property_descriptor::{CompositeDescriptor, IntegerFormat, PropertyDescriptor, StringDescriptor};
use types_descriptor::type_header::{BaseType, SemanticVersion, TypeHeader};

pub struct PropertyMapBuilder {
    properties: BTreeMap<String, PropertyDescriptor>,
}

impl PropertyMapBuilder {
    // Creates a new (dependent) StringDescriptor and adds it to the PropertyMap
    pub fn add_string_property(
        &mut self,
        property_name: String,
        type_name: String,
        description: String,
        is_dependent: bool,
        min_length: u32,
        max_length: u32,
    ) -> ExternResult<&mut Self> {
        // For now, we are ONLY supporting dedicated PropertyDescriptors
        // So this method always creates a new PropertyDescriptor

        let descriptor = new_string_descriptor(
            type_name,
            description,
            is_dependent,
            min_length,
            max_length,
        )?;

        self.properties.insert(property_name, descriptor);

        Ok(self)
    }


    // Creates a new (dependent) IntegerDescriptor and adds it to the PropertyMap
    pub fn add_integer_property(
        &mut self,
        property_name: String,
        type_name: String,
        description: String,
        is_dependent: bool,
        format: IntegerFormat,
        min_value: u128,
        max_value: u128,
    ) -> ExternResult<&mut Self> {
        // For now, we are ONLY supporting dedicated PropertyDescriptors
        // So this method always creates a new PropertyDescriptor

        let descriptor = new_integer_descriptor(
            type_name,
            description,
            is_dependent,
            format,
            min_value,
            max_value,
        )?;

        self.properties.insert(property_name, descriptor);

        Ok(self)
    }


    // Creates a new (dependent) StringDescriptor and adds it to the PropertyMap
    pub fn add_boolean_property(
        &mut self,
        property_name: String,
        type_name: String,
        description: String,
        is_dependent: bool,
        is_fuzzy: bool,
    ) -> ExternResult<&mut Self> {
        // For now, we are ONLY supporting dedicated PropertyDescriptors
        // So this method always creates a new PropertyDescriptor

        let descriptor = new_boolean_descriptor(
            type_name,
            description,
            is_dependent,
            is_fuzzy,
        )?;

        self.properties.insert(property_name, descriptor);

        Ok(self)
    }

}
/*
add_integer_property(&self, property_name, type_name, description, format, min_value, max_value)->ExternResult<PropertyMapBuilder>
add_boolean_property(&self, property_name, type_name, description, is_fuzzy)->ExternResult<PropertyMapBuilder>
add_composite_property(&self, property_name, type_name, description, composite:CompositeDescriptor)->ExternResult<PropertyMapBuilder>
add_value_collection_property(&self, property_name, type_name, description, collection:ValueCollectionDescriptor)->ExternResult<PropertyMapBuilder>

*/

