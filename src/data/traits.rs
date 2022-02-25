use std::hash::Hash;

use strum::ParseError;

use super::enums::{DSPItem, DSPRecipe};

pub trait DSPEnum: Eq + Copy + Hash + for<'a> TryFrom<&'a str, Error = ParseError> +
                   TryFrom<Self::Underlying> + Into<Self::Underlying>
{
    type Underlying: Copy;
}
impl DSPEnum for DSPRecipe {
    type Underlying = u16;
}
impl DSPEnum for DSPItem {
    type Underlying = u16;
}

pub type Replace<T> = dyn Fn(T) -> T;

pub trait ReplaceRecipe {
    fn replace_recipe(&mut self, replace: &Replace<DSPRecipe>);
}

pub trait ReplaceItem {
    fn replace_item(&mut self, replace: &Replace<DSPItem>);
}

macro_rules! from_into_boilerplate {
    ($t: ty, $ul: ty, $enum: ty) => {
        impl From<$enum> for $t {
            fn from(i: $enum) -> Self {
                <$ul>::from(i) as $t
            }
        }

        impl TryInto<$enum> for $t {
            type Error = anyhow::Error;

            fn try_into(self) -> Result<$enum, Self::Error> {
                Ok(<$ul>::try_from(self)?.try_into()?)
            }
        }
    }
}

from_into_boilerplate!(u32, u16, DSPItem);
from_into_boilerplate!(u32, u16, DSPRecipe);

impl<T: TryInto<DSPItem> + From<DSPItem> + Copy> ReplaceItem for T
{
    fn replace_item(&mut self, replace: &Replace<DSPItem>) {
        let my_item = match (*self).try_into() {
            Ok(l) => l,
            _ => return,
        };
        *self = replace(my_item).into();
    }
}

impl<T: TryInto<DSPRecipe> + From<DSPRecipe> + Copy> ReplaceRecipe for T
{
    fn replace_recipe(&mut self, replace: &Replace<DSPRecipe>) {
        let my_item = match (*self).try_into() {
            Ok(l) => l,
            _ => return,
        };
        *self = replace(my_item).into();
    }
}
