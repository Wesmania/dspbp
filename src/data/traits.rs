use std::hash::Hash;

use strum::ParseError;

use super::enums::{DSPItem, DSPRecipe};

pub type Replace<T> = dyn Fn(T) -> T;

// Evil generics. I want to convert u16/u32 to enums without repeating myself.
pub trait DSPEnum: Eq + Copy + Hash + for<'a> TryFrom<&'a str, Error = ParseError> + TryFrom<Self::Underlying> + Into<Self::Underlying> {
    type Underlying: Copy;
}

impl DSPEnum for DSPRecipe {
    type Underlying = u16;
}
impl DSPEnum for DSPItem {
    type Underlying = u16;
}

// Can't use DSPEnum twice with disjoint types as params, so here's a workaround
pub trait DSPEnumSpec<Underlying: Copy>: Eq + Copy + TryFrom<Underlying> + Into<Underlying> {}
impl<T: DSPEnum> DSPEnumSpec<T::Underlying> for T {}

pub trait ReplaceEnum<Underlying: Copy, Enum: DSPEnumSpec<Underlying>> {
    fn replace_enum(&mut self, replace: &Replace<Enum>);
}

impl<Underlying: Copy, Enum: DSPEnumSpec<Underlying>> ReplaceEnum<Underlying, Enum> for Underlying {
    fn replace_enum(&mut self, replace: &Replace<Enum>) {
        match Enum::try_from(*self) {
            Err(_) => return,
            Ok(e) => {
                *self = replace(e).into();
            }
        }
    }
}

impl <Enum: DSPEnumSpec<u16>> ReplaceEnum<u16, Enum> for u32 {
    fn replace_enum(&mut self, replace: &Replace<Enum>) {
        let mut label_u16 = match u16::try_from(*self) {
            Ok(l) => l,
            _ => return,
        };
        label_u16.replace_enum(replace);
        *self = label_u16 as u32;
    }
}

pub trait ReplaceItem {
    fn replace_item(&mut self, replace: &Replace<DSPItem>);
}

impl<Type> ReplaceItem for Type where
    Type: ReplaceEnum<u16, DSPItem>,
{
    fn replace_item(&mut self, replace: &Replace<DSPItem>) {
        self.replace_enum(replace)
    }
}

pub trait ReplaceRecipe {
    fn replace_recipe(&mut self, replace: &Replace<DSPRecipe>);
}

impl<Type> ReplaceRecipe for Type where
    Type: ReplaceEnum<u16, DSPRecipe>,
{
    fn replace_recipe(&mut self, replace: &Replace<DSPRecipe>) {
        self.replace_enum(replace)
    }
}
