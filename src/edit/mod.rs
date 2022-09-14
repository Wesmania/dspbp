use std::collections::HashMap;

use anyhow::Result;

use crate::{
    blueprint::Blueprint,
    data::{
        enums::{BuildingClass, DSPItem, DSPRecipe},
        traits::DSPEnum,
        visit::Visitor,
    },
};

use self::{
    replace::{Replace, ReplaceBuilding, ReplaceItem, ReplaceRecipe},
    stats::GetStats,
};

pub(crate) mod replace;
pub(crate) mod stats;

fn map_using_map<T: DSPEnum + 'static>(m: HashMap<T, T>) -> Box<Replace<T>> {
    Box::new(move |from| *m.get(&from).unwrap_or(&from))
}

pub struct EditBlueprint(pub Blueprint);

impl EditBlueprint {
    pub fn new(bp: Blueprint) -> Self {
        Self(bp)
    }

    pub fn get_icon_text(&self) -> Result<String> {
        self.0.get_icon_text()
    }

    pub fn set_icon_text(&mut self, test: &str) {
        self.0.set_icon_text(&test);
    }

    pub fn get_description(&self) -> Result<String> {
        self.0.get_description()
    }

    pub fn info(&mut self) -> Result<String> {
        let mut stats = GetStats::new();
        stats.visit_blueprint(&mut self.0);
        Ok(format!("{}", stats.0))
    }

    pub fn replace_item(&mut self, map: HashMap<DSPItem, DSPItem>) {
        let m = map_using_map(map);
        let mut r = ReplaceItem::new(&m);
        r.visit_blueprint(&mut self.0);
    }

    pub fn replace_recipe(&mut self, map: HashMap<DSPRecipe, DSPRecipe>) {
        let m = map_using_map(map);
        let mut r = ReplaceRecipe::new(&m);
        r.visit_blueprint(&mut self.0);
    }

    pub fn replace_both(&mut self, r: HashMap<DSPItem, DSPItem>) {
        let r2: HashMap<DSPRecipe, DSPRecipe> = r
            .iter()
            .filter_map(|(k, v)| {
                let k = DSPRecipe::for_item(k)?;
                let v = DSPRecipe::for_item(v)?;
                Some((k, v))
            })
            .collect();
        self.replace_item(r);
        self.replace_recipe(r2);
    }

    pub fn replace_building(&mut self, map: HashMap<DSPItem, DSPItem>) -> Result<()> {
        map.iter().try_for_each(|(i, o)| {
            if !BuildingClass::replacement_is_valid(*i, *o) {
                let e: crate::error::Error =
                    format!("Cannot replace buildings: {} -> {}", i.as_ref(), o.as_ref()).into();
                Err(anyhow::Error::from(e))
            } else {
                Ok(())
            }
        })?;
        let m = map_using_map(map);
        let mut r = ReplaceBuilding::new(&m);
        r.visit_blueprint(&mut self.0);
        Ok(())
    }
}
