use serde::{Deserialize, Serialize};
use struct_deser_derive::StructDeser;

use crate::{
    data::{area::Area, building::Building},
    error::Error,
    serialize::{Deser, Ser},
};

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
    fn from_bp(d: &mut Deser) -> anyhow::Result<Self> {
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

    fn to_bp(&self, d: &mut Ser) -> anyhow::Result<()> {
        d.write_type(&self.header);
        for a in &self.areas {
            d.write_type(a);
        }
        d.write_type(&self.building_count);
        for b in &self.buildings {
            b.to_bp(d)?;
        }
        Ok(())
    }

    pub fn new_from_buf(b: &[u8]) -> anyhow::Result<Self> {
        Self::from_bp(&mut Deser::new(b))
    }

    pub fn write(&self) -> anyhow::Result<Vec<u8>> {
        let mut w = Ser::new();
        self.to_bp(&mut w)?;
        Ok(w.data())
    }
}
