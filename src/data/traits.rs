use std::hash::Hash;

use strum::ParseError;

use binrw::{BinWrite, BinRead};
#[cfg(feature = "dump")]
use serde::{Deserialize, Serialize};

use super::enums::{DSPItem, DSPRecipe, DSPIcon, BPModel};

pub trait DSPEnum:
    Eq
    + Copy
    + Hash
    + for<'a> TryFrom<&'a str, Error = ParseError>
    + TryFrom<Self::Underlying>
    + Into<Self::Underlying>
{
    type Underlying: Copy;
    const PRETTY_NAME: &'static str;
}

pub trait TryFromUserString: Sized {
    fn try_from_user_string(s: &str) -> anyhow::Result<Self>;
}

impl<T: DSPEnum> TryFromUserString for T {
    fn try_from_user_string(s: &str) -> anyhow::Result<Self> {
        Self::try_from(s).or_else(|_| anyhow::bail!("'{}' is not a known {}. Run 'dspbp items' or 'dspbp recipes' for a list of item/recipe names.", s, T::PRETTY_NAME))
    }
}

impl DSPEnum for DSPRecipe {
    type Underlying = u16;
    const PRETTY_NAME: &'static str = "recipe";
}
impl DSPEnum for DSPItem {
    type Underlying = u16;
    const PRETTY_NAME: &'static str = "item";
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

// These are newtypes for various u16/u32 values in the blueprint. Help make sure we don't misuse
// them and will allow for better localization in the future.

pub trait Nice: for<'a> BinWrite<Args<'a> = ()> + for<'b> BinRead<Args<'b> = ()> + core::fmt::Debug + PartialEq + Eq + Clone + Copy {}
impl<T> Nice for T
where T: for<'a> BinWrite<Args<'a> = ()> + for<'b> BinRead<Args<'b> = ()> + core::fmt::Debug + PartialEq + Eq + Clone + Copy {}

macro_rules! newtype_enum {
    ($DSP: ty, $Id: ident) => {
        #[cfg_attr(feature = "dump", derive(Serialize, Deserialize))]
        #[derive(BinRead, BinWrite, Debug, PartialEq, Eq, Clone, Copy)]
        pub struct $Id<T: Nice + TryInto<$DSP> + From<$DSP>>(pub T);

        impl<T: Nice + TryInto<$DSP> + From<$DSP>> TryFrom<$Id<T>> for $DSP {
            type Error = T::Error;

            fn try_from(value: $Id<T>) -> Result<Self, Self::Error> {
                value.0.try_into()
            }
        }

        impl<T: Nice + TryInto<$DSP> + From<$DSP>> From<$DSP> for $Id<T> {
            fn from(value: $DSP) -> Self {
                Self(value.into())
            }
        }
    }
}

newtype_enum!(DSPItem, ItemId);
newtype_enum!(DSPRecipe, RecipeId);
newtype_enum!(DSPIcon, IconId);
newtype_enum!(BPModel, BPModelId);
