use hdk::prelude::*;
use types_descriptor::descriptor::{HolonDescriptor,TypeHeader, BaseType, SemanticVersion};

#[hdk_extern]
pub fn get_all_holontypes(_:()) -> ExternResult<Vec<HolonDescriptor>> {
    let descriptor1 = HolonDescriptor {
        header: Box::new(TypeHeader::new("holonType1".to_string(),
                                         "desc1".to_string(),
        )),
    };
/*
    let descriptor2 = HolonDescriptor {
        header: Box::new(TypeHeader::new("holonType2".to_string(), "desc2".to_string())),
    };

    let descriptor3 = HolonDescriptor {
        header: Box::new(TypeHeader::new("holonType3".to_string(), "desc3".to_string())),
    };
*/
    let descriptors_vec = vec![descriptor1];

    Ok(descriptors_vec)
}

