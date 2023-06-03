
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
fn create_property_descriptor(header: Box<TypeHeader>, constraints: Box<PropertyTypeConstraint>)-> ExternResult<Box<PropertyDescriptor>> {
    Ok(Box::new(PropertyDescriptor::new( header, constraints)))
    }

impl HolonDescriptor {
    pub fn add_property_descriptor(
        &mut self,
        property_name: String,
        base_type: BaseType,
        type_name: String,
        description: String,
        is_dependent: bool,
        constraints: Box<PropertyTypeConstraint>,
    ) -> ExternResult<Self> {
        // TODO: Add guard check that base_type = constraints type

        // Create a TypeHeader
        let header = create_type_header(
            type_name, base_type, description, is_dependent: bool,
        )?;
        // Create a PropertyTypeDescriptor that includes that TypeHeader
        let property_descriptor : PropertyDescriptor = create_property_descriptor(header, constraints)?;

        // Add the new PropertyTypeDescriptor to the properties B-Tree
        self.properties.insert(property_name, property_descriptor);
        Ok(self.clone())
    }
}
*/

