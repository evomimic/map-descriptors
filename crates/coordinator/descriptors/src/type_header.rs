use hdi::prelude::ExternResult;
use types_descriptor::descriptor::{BaseType, SemanticVersion, TypeHeader};

pub fn create_type_header(
    type_name: String,
    description: String,
    is_dependent: bool,
) -> ExternResult<TypeHeader> {
    let header = TypeHeader::new(
        type_name,
        BaseType::Holon,
        description,
        SemanticVersion::default(),
        is_dependent,
    );

    Ok(header)
}
