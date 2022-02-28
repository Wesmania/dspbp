use binrw::{BinWrite, BinRead};
#[cfg(feature = "dump")]
use serde::{Deserialize, Serialize};

#[cfg_attr(feature = "dump", derive(Serialize, Deserialize))]
#[derive(BinRead, BinWrite)]
pub struct Area {
    index: i8,
    parent_index: i8,
    #[br(little)]
    tropic_anchor: u16,
    #[br(little)]
    area_segments: u16,
    #[br(little)]
    anchor_local_offset_x: u16,
    #[br(little)]
    anchor_local_offset_y: u16,
    #[br(little)]
    width: u16,
    #[br(little)]
    height: u16,
}
