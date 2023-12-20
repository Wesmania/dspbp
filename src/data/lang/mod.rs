#[cfg(lang_zh)]
mod lang_zh;
#[cfg(lang_zh)]
pub use lang_zh::{ITEM_DATA, RECIPE_DATA};
#[cfg(lang_en)]
mod lang_en;
#[cfg(lang_en)]
pub use lang_en::{ITEM_DATA, RECIPE_DATA};



#[cfg(all(lang_zh, lang_en))]
compiler_error!("you could not choose both `lang_zh` and `lang_en`");
#[cfg(all(not(lang_zh),not(lang_en)))]
const ITEM_DATA:&[(u16,&str)]=&[];
const RECIPE_DATA:&[(u16,&str)]=&[];

use serde::{Deserialize, Serialize};
static ID_NAME: std::sync::LazyLock<std::collections::HashMap<u16, &'static str>> =
    std::sync::LazyLock::new(|| std::collections::HashMap::from_iter(ITEM_DATA.iter().copied()));
static NAME_ID: std::sync::LazyLock<std::collections::HashMap<&'static str, u16>> =
    std::sync::LazyLock::new(|| {
        std::collections::HashMap::from_iter(ITEM_DATA.iter().map(|&(a, b)| (b, a)))
    });

pub fn ser_item_id<S: serde::Serializer>(&item: &u16, serializer: S) -> Result<S::Ok, S::Error> {
    serializer.serialize_str(
        ID_NAME
            .get(&item)
            .copied()
            .unwrap_or(item.to_string().as_str()),
    )
}
pub fn de_item_id<'de, D: serde::Deserializer<'de>>(de: D) -> Result<u16, D::Error> {
    let id = String::deserialize(de)?;
    Ok(NAME_ID.get(id.as_str()).map(|x| x.clone()).unwrap_or(
        id.parse::<u16>().map(|x| x.into()).unwrap_or_else(|_| {
            if let Some(x) = id.rsplit('(').next() {
                x.split(')') // add support for unknown ids generated from newer dumper
                    .next()
                    .map(|x| x.parse().unwrap_or(u16::MAX))
                    .unwrap_or(u16::MAX)
            } else {
                u16::MAX
            }
        })
    ))
}
