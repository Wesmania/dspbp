use std::io::{Read, Write};

use serde::{Deserialize, Serialize};
use struct_deser_derive::StructDeser;

use crate::{
    data::{area::Area, building::Building},
    error::Error, serialize::{ReadType, WriteType}, stats::{GetStats, Stats},
};

use super::{traits::{ReplaceItem, ReplaceRecipe, Replace}, enums::{DSPItem, DSPRecipe}};

#[derive(Serialize, Deserialize, StructDeser)]
pub struct Header {
    #[le]
    version: u32,
    #[le]
    cursor_offset_x: u32,
    #[le]
    cursor_offset_y: u32,
    #[le]
    cursor_target_area: u32,
    #[le]
    dragbox_size_x: u32,
    #[le]
    dragbox_size_y: u32,
    #[le]
    primary_area_index: u32,
    area_count: u8,
}

#[derive(Serialize, Deserialize, StructDeser)]
pub struct BuildingCount(#[le] u32);

#[derive(Serialize, Deserialize)]
pub struct BlueprintData {
    header: Header,
    areas: Vec<Area>,
    building_count: BuildingCount,
    buildings: Vec<Building>,
}

impl BlueprintData {
    pub fn from_bp(mut d: &mut dyn Read) -> anyhow::Result<Self> {
        let header: Header = d.read_type()?;
        if header.version != 1 {
            return Err(Error::E(format!(
                "Expected blueprint version 1, got {}",
                header.version
            ))
            .into());
        }
        let mut areas = vec![];
        let mut buildings = vec![];
        for _ in 0..header.area_count {
            areas.push(d.read_type()?);
        }
        let building_count: BuildingCount = d.read_type()?;
        for _ in 0..building_count.0 {
            buildings.push(Building::from_bp(d)?);
        }
        Ok(Self {
            header,
            areas,
            building_count,
            buildings,
        })
    }

    pub fn to_bp(&self, mut d: &mut dyn Write) -> anyhow::Result<()> {
        d.write_type(&self.header)?;
        for a in &self.areas {
            d.write_type(a)?;
        }
        d.write_type(&self.building_count)?;
        for b in &self.buildings {
            b.to_bp(d)?;
        }
        Ok(())
    }
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
