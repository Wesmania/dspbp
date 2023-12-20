use binrw::{BinRead, BinWrite};
use num_enum::TryFromPrimitiveError;
#[cfg(feature = "dump")]
use serde::{Deserialize, Serialize};

use super::{
    belt::Belt,
    enums::DSPItem,
    station::Station,
    visit::{Visit, Visitor}, traits::{ItemId, BPModelId, RecipeId},
};

fn b_is(i: ItemId<u16>, f: fn(&DSPItem) -> bool) -> bool {
    i.try_into().as_ref().map(f).unwrap_or(false)
}

#[cfg_attr(feature = "dump", derive(Serialize, Deserialize))]
#[derive(BinRead, BinWrite)]
#[br(import { param_count: usize, building: ItemId<u16> })]
#[br(pre_assert(param_count <= 32768))]
pub enum BuildingParam {
    #[br(pre_assert(b_is(building, DSPItem::is_station)))]
    Station(
        #[br(args { is_interstellar: b_is(building, DSPItem::is_interstellar_station), param_count: param_count })]
         Station,
    ),
    #[br(pre_assert(b_is(building, DSPItem::is_belt)))]
    Belt(
        #[br(if(param_count != 0))]
        #[br(args(param_count))]
        Option<Belt>,
    ),
    Unknown(
        #[br(count = param_count)]
        #[br(little)]
        Vec<u32>,
    ),
}

#[cfg_attr(feature = "dump", derive(Serialize, Deserialize))]
#[derive(BinRead, BinWrite)]
pub struct BuildingHeader {
    #[br(little)]
    pub index: u32,
    pub area_index: i8,
    #[br(little)]
    pub local_offset_x: f32,
    #[br(little)]
    pub local_offset_y: f32,
    #[br(little)]
    pub local_offset_z: f32,
    #[br(little)]
    pub local_offset_x2: f32,
    #[br(little)]
    pub local_offset_y2: f32,
    #[br(little)]
    pub local_offset_z2: f32,
    #[br(little)]
    pub yaw: f32,
    #[br(little)]
    pub yaw2: f32,
    #[br(little)]
    pub item_id: ItemId<u16>,
    #[br(little)]
    pub model_index: BPModelId<u16>,
    #[br(little)]
    pub output_object_index: u32,
    #[br(little)]
    pub input_object_index: u32,
    pub output_to_slot: i8,
    pub input_from_slot: i8,
    pub output_from_slot: i8,
    pub input_to_slot: i8,
    pub output_offset: i8,
    pub input_offset: i8,
    #[br(little)]
    pub recipe_id: RecipeId<u16>,
    #[br(little)]
    pub filter_id: ItemId<u16>,
    #[br(little)]
    pub parameter_count: u16,
}

#[cfg_attr(feature = "dump", derive(Serialize, Deserialize))]
#[derive(BinRead, BinWrite)]
pub struct Building {
    pub header: BuildingHeader,
    #[br(args { param_count: header.parameter_count as usize, building: header.item_id })]
    pub param: BuildingParam,
}

impl Building {
    pub fn kind(&self) -> Result<DSPItem, TryFromPrimitiveError<DSPItem>> {
        DSPItem::try_from(self.header.item_id)
    }
}

impl Visit for Building {
    fn visit<T: Visitor + ?Sized>(&mut self, visitor: &mut T) {
        match &mut self.param {
            BuildingParam::Station(s) => visitor.visit_station(s),
            BuildingParam::Belt(Some(b)) => visitor.visit_belt(b),
            _ => (),
        }
    }
}
