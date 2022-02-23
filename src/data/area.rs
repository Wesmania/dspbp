use serde::{Deserialize, Serialize};
use struct_deser_derive::StructDeser;

#[derive(Serialize, Deserialize, StructDeser)]
pub struct Area {
    index: i8,
    parent_index: i8,
    #[le]
    tropic_anchor: u16,
    #[le]
    area_segments: u16,
    #[le]
    anchor_local_offset_x: u16,
    #[le]
    anchor_local_offset_y: u16,
    #[le]
    width: u16,
    #[le]
    height: u16,
}
