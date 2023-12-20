use std::{sync::OnceLock, collections::HashMap};

use crate::data::{enums::{DSPItem, DSPRecipe, BPModel, DSPIcon}};
use lazy_static::lazy_static;

#[allow(non_camel_case_types)]
#[derive(PartialEq, Eq, Hash)]
pub(crate) enum Locale {
    None,
    en_en,
    zh_cn,
}

pub(crate) static GLOBAL_SERIALIZATION_LOCALE: OnceLock<Locale> = OnceLock::new();

pub(crate) trait LocalizedEnum: std::hash::Hash + Eq + PartialEq + Sized {
    fn get_locale_table(&self) -> &'static HashMap<Locale, HashMap<Self, &'static str>>;
}

pub(crate) trait LocalizedEnumImpl {
    fn localize(&self) -> Option<&'static str>;
}

impl<T: LocalizedEnum + 'static> LocalizedEnumImpl for T {
    fn localize(&self) -> Option<&'static str> {
        let global_locale = GLOBAL_SERIALIZATION_LOCALE.get()?;
        Some(*self.get_locale_table().get(global_locale)?.get(self)?)
    }
}

macro_rules! localized_enum_impl {
    ($enum: ty, $table: ident) => {
        lazy_static! {
            static ref $table: HashMap<Locale, HashMap<$enum, &'static str>> = HashMap::from_iter([]);
        }

        impl LocalizedEnum for $enum {
            fn get_locale_table(&self) -> &'static HashMap<Locale, HashMap<Self, &'static str>> {
                &*$table
            }
        }
    }
}

localized_enum_impl!(DSPItem, DSP_ENUM_LOCALE);
localized_enum_impl!(DSPRecipe, DSP_RECIPE_LOCALE);
localized_enum_impl!(BPModel, BP_MODEL_LOCALE);

impl LocalizedEnumImpl for DSPIcon {
    fn localize(&self) -> Option<&'static str> {
        match self {
            DSPIcon::Signal(_) => None,
            DSPIcon::Item(i) => i.localize(),
            DSPIcon::Recipe(i) => i.localize(),
            DSPIcon::Tech(_) => None,
            DSPIcon::Unknown(_) => None,
        }
    }
}
