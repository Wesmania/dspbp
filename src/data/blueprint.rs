use std::io::Cursor;

use binrw::{BinWrite, BinRead, BinReaderExt};
#[cfg(feature = "dump")]
use serde::{Deserialize, Serialize};


use crate::{
    data::{area::Area, building::Building},
    error::Error, stats::{GetStats, Stats}, ReadPlusSeek,
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
pub struct BuildingCount(#[br(little)] u32);

#[cfg_attr(feature = "dump", derive(Serialize, Deserialize))]
pub struct BlueprintData {
    header: Header,
    areas: Vec<Area>,
    building_count: BuildingCount,
    buildings: Vec<Building>,
}

impl BlueprintData {
    pub fn from_bp(mut d: &mut dyn ReadPlusSeek) -> anyhow::Result<Self> {
        let header: Header = d.read_le()?;
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
            areas.push(d.read_le()?);
        }
        let building_count: BuildingCount = d.read_le()?;
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

    pub fn to_bp(&self, d: &mut Cursor<Vec<u8>>) -> anyhow::Result<()> {
        self.header.write_to(d)?;
        for a in &self.areas {
            a.write_to(d)?;
        }
        self.building_count.write_to(d)?;
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
