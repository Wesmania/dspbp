use binrw::{BinWrite, BinRead};
#[cfg(feature = "dump")]
use serde::{Deserialize, Serialize};


use crate::{
    data::{area::Area, building::Building},
    stats::{GetStats, Stats},
};

use super::{traits::{ReplaceItem, ReplaceRecipe, Replace}, enums::{DSPItem, DSPRecipe}};

#[cfg_attr(feature = "dump", derive(Serialize, Deserialize))]
#[derive(BinRead, BinWrite)]
pub struct Header {
    #[br(little)]
    version: u32,
    #[br(little)]
    cursor_offset_x: u32,
    #[br(little)]
    cursor_offset_y: u32,
    #[br(little)]
    cursor_target_area: u32,
    #[br(little)]
    dragbox_size_x: u32,
    #[br(little)]
    dragbox_size_y: u32,
    #[br(little)]
    primary_area_index: u32,
    area_count: u8,
}

#[cfg_attr(feature = "dump", derive(Serialize, Deserialize))]
#[derive(BinRead, BinWrite)]
pub struct BlueprintData {
    #[br(assert(header.version == 1))]
    pub header: Header,
    #[br(count = header.area_count)]
    pub areas: Vec<Area>,
    #[br(little)]
    pub building_count: u32,
    #[br(count = building_count)]
    pub buildings: Vec<Building>,
}

impl ReplaceItem for BlueprintData {
    fn replace_item(&mut self, replace: &Replace<DSPItem>) {
        for building in &mut self.buildings {
            building.replace_item(replace)
        }
    }
}

impl ReplaceRecipe for BlueprintData {
    fn replace_recipe(&mut self, replace: &Replace<DSPRecipe>) {
        for building in &mut self.buildings {
            building.replace_recipe(replace)
        }
    }
}

impl GetStats for BlueprintData {
    fn get_stats(&self, stats: &mut Stats) {
        for building in &self.buildings {
            building.get_stats(stats)
        }
    }
}
