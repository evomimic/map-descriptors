/// value_types: ome shared primitive types
///
use hdi::prelude::*;
use derive_new::new;

#[hdk_entry_helper]
#[derive(new, Clone, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct UnitInterval {
    value: u16, // value can range from 0 to 1, inclusive
}

/// FuzzyBoolean allows degrees of certainty between true and false.
///
/// Typically, values would be expressed using a floating point number with a value
/// between 0 and 1 inclusive.
///
/// But because Eq trait is not implemented for f32, it is problematic for UnitInterval to be
/// a floating point number. Trying to use a u16 instead here.
/// Values can range from 0 to 65,535
/// zero = false, >0 = (somewhat) true,  65,535 = certain

#[hdk_entry_helper]
#[derive(new, Clone, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
struct FuzzyBoolean {
    value: UnitInterval,
    }






