use std::{collections::HashMap, sync::OnceLock};

use crate::data::{
    enums::{BPModel, DSPIcon, DSPItem, DSPRecipe},
    traits::TryFromUserString,
};
use lazy_static::lazy_static;
use strum::{EnumIter, EnumString, IntoEnumIterator, IntoStaticStr};

#[allow(non_camel_case_types)]
#[derive(PartialEq, Eq, Hash, EnumString, EnumIter, IntoStaticStr, Clone, Copy)]
pub(crate) enum Locale {
    en,
    cn,
}

impl TryFromUserString for Locale {
    fn try_from_user_string(s: &str) -> anyhow::Result<Self> {
        Self::try_from(s).or_else(|_| {
            let locales = Self::iter()
                .map(|e| <&'static str>::from(e))
                .collect::<Vec<_>>()
                .join(", ");
            anyhow::bail!("Unknown locale '{}'. Supported locales: {}.", s, locales);
        })
    }
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

struct LList<T: 'static>(Locale, &'static [(T, &'static str)]);

static DSP_ITEM_LLIST: &[LList<DSPItem>] = &[
    LList(Locale::en, include!("data/en/items.rs")),
    LList(Locale::cn, include!("data/cn/items.rs")),
];

static DSP_RECIPE_LLIST: &[LList<DSPRecipe>] = &[
    LList(Locale::en, include!("data/en/recipes.rs")),
    LList(Locale::cn, include!("data/cn/recipes.rs")),
];
// FIXME how to extract model names from DSP? Console has no commands for that.
static DSP_MODEL_LLIST: &[LList<BPModel>] = &[];

macro_rules! localized_enum_impl {
    ($enum: ty, $table: ident, $source: ident) => {
        lazy_static! {
            static ref $table: HashMap<Locale, HashMap<$enum, &'static str>> = {
                let mut map = HashMap::new();
                for locale in $source {
                    let mut submap = HashMap::new();
                    for pair in locale.1 {
                        submap.insert(pair.0, pair.1);
                    }
                    map.insert(locale.0, submap);
                }
                map
            };
        }

        impl LocalizedEnum for $enum {
            fn get_locale_table(&self) -> &'static HashMap<Locale, HashMap<Self, &'static str>> {
                &*$table
            }
        }
    };
}

localized_enum_impl!(DSPItem, DSP_ENUM_LOCALE, DSP_ITEM_LLIST);
localized_enum_impl!(DSPRecipe, DSP_RECIPE_LOCALE, DSP_RECIPE_LLIST);
localized_enum_impl!(BPModel, BP_MODEL_LOCALE, DSP_MODEL_LLIST);

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
