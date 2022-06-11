use crate::data::enums::{DSPItem, DSPRecipe};
use std::{collections::HashMap, fmt::Display, hash::Hash};

#[derive(Default)]
pub struct Stats {
    pub buildings: HashMap<DSPItem, usize>,
    pub recipes: HashMap<DSPRecipe, usize>,
    pub station_wares: HashMap<DSPItem, usize>,
}

impl Stats {
    pub fn new() -> Self {
        Default::default()
    }

    fn incmap<T: Eq + Hash>(map: &mut HashMap<T, usize>, i: T) {
        match map.get_mut(&i) {
            None => {
                map.insert(i, 1);
            }
            Some(n) => *n += 1,
        }
    }

    fn printmap<T: Eq + Hash + AsRef<str>>(
        f: &mut std::fmt::Formatter<'_>,
        map: &HashMap<T, usize>,
    ) -> std::fmt::Result {
        for (item, count) in map {
            if *count == 0 {
                continue;
            }
            writeln!(f, "{}: {}", item.as_ref(), count)?;
        }
        Ok(())
    }

    pub fn add_building(&mut self, i: DSPItem) {
        Self::incmap(&mut self.buildings, i)
    }

    pub fn add_recipe(&mut self, i: DSPRecipe) {
        Self::incmap(&mut self.recipes, i)
    }

    pub fn add_station_ware(&mut self, i: DSPItem) {
        Self::incmap(&mut self.station_wares, i)
    }
}

impl Display for Stats {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "Buildings:")?;
        Self::printmap(f, &self.buildings)?;
        writeln!(f, "")?;
        writeln!(f, "Recipes:")?;
        Self::printmap(f, &self.recipes)?;
        writeln!(f, "")?;
        writeln!(f, "Logistic station wares:")?;
        Self::printmap(f, &self.station_wares)?;
        Ok(())
    }
}
