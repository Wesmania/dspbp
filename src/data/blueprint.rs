use binrw::{BinRead, BinWrite};
#[cfg(feature = "dump")]
use serde::{Deserialize, Serialize};

use crate::data::{area::Area, building::Building};

use super::visit::{Visit, Visitor};

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

impl Visit for BlueprintData {
    fn visit<T: Visitor + ?Sized>(&mut self, visitor: &mut T) {
        for b in self.buildings.iter_mut() {
            visitor.visit_building(b)
        }
    }
}
