use std::hash::Hash;

use strum::ParseError;

use super::enums::{DSPItem, DSPRecipe};

pub trait DSPEnum:
    Eq
    + Copy
    + Hash
    + for<'a> TryFrom<&'a str, Error = ParseError>
    + TryFrom<Self::Underlying>
    + Into<Self::Underlying>
{
    type Underlying: Copy;
}
impl DSPEnum for DSPRecipe {
    type Underlying = u16;
}
impl DSPEnum for DSPItem {
    type Underlying = u16;
}

macro_rules! from_into_boilerplate {
    ($t: ty, $ul: ty, $enum: ty) => {
        impl From<$enum> for $t {
            fn from(i: $enum) -> Self {
                <$ul>::from(i) as $t
            }
        }

        impl TryFrom<$t> for $enum {
            type Error = anyhow::Error;

            fn try_from(v: $t) -> Result<Self, Self::Error> {
                Ok(<$ul>::try_from(v)?.try_into()?)
            }
        }
    };
}

from_into_boilerplate!(u32, u16, DSPItem);
from_into_boilerplate!(u32, u16, DSPRecipe);
