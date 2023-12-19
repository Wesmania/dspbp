use binrw::{BinRead, BinWrite};
use num_enum::TryFromPrimitiveError;
#[cfg(feature = "dump")]
use serde::{Deserialize, Serialize};

use super::{
    belt::Belt,
    enums::DSPItem,
    station::Station,
    visit::{Visit, Visitor},
};

fn b_is(i: u16, f: fn(&DSPItem) -> bool) -> bool {
    i.try_into().as_ref().map(f).unwrap_or(false)
}

#[cfg_attr(feature = "dump", derive(Serialize, Deserialize))]
#[derive(BinRead, BinWrite)]
#[br(import { param_count: usize, building: u16 })]
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

pub static mut RD20: bool = true;
pub static mut RM5: bool = true;
fn round_loc(f: f32) -> f64 {
    if unsafe { RD20 } {
        ((f as f64) * 20.).round() / 20.
    } else {
        f as f64
    }
}
fn round_yew(f: f32) -> f64 {
    if unsafe { RM5 } {
        ((f as f64) / 5.).round() * 5.
    } else {
        f as f64
    }
}
fn to_f32(f: &f64) -> f32 {
    *f as f32
}
const DYSON_DATA: &'static [(u16, &'static str)] = &include!("DYSON_DATA.rs");
static ID_NAME: std::sync::LazyLock<std::collections::HashMap<u16, &'static str>> =
    std::sync::LazyLock::new(|| std::collections::HashMap::from_iter(DYSON_DATA.iter().copied()));
static NAME_ID: std::sync::LazyLock<std::collections::HashMap<&'static str, u16>> =
    std::sync::LazyLock::new(|| {
        std::collections::HashMap::from_iter(DYSON_DATA.iter().map(|&(a, b)| (b, a)))
    });
fn ser_item_id<S: serde::Serializer>(&item: &u16, serializer: S) -> Result<S::Ok, S::Error> {
    serializer.serialize_str(
        ID_NAME
            .get(&item)
            .copied()
            .unwrap_or(item.to_string().as_str()),
    )
}
fn de_item_id<'de, D: serde::Deserializer<'de>>(de: D) -> Result<u16, D::Error> {
    let id = String::deserialize(de)?;
    Ok(NAME_ID
        .get(id.as_str())
        .map(|x| x.clone())
        .unwrap_or(id.parse().unwrap_or(u16::MAX)))
}

#[cfg_attr(feature = "dump", derive(Serialize, Deserialize))]
#[derive(BinRead, BinWrite)]
pub struct BuildingHeader {
    #[br(little)]
    pub index: u32,
    pub area_index: i8,
    #[br(map = round_loc)]
    #[bw(map = to_f32)]
    pub local_offset_x: f64,
    #[br(map = round_loc)]
    #[bw(map = to_f32)]
    pub local_offset_y: f64,
    #[br(map = round_loc)]
    #[bw(map = to_f32)]
    pub local_offset_z: f64,
    #[br(map = round_loc)]
    #[bw(map = to_f32)]
    pub local_offset_x2: f64,
    #[br(map = round_loc)]
    #[bw(map = to_f32)]
    pub local_offset_y2: f64,
    #[br(map = round_loc)]
    #[bw(map = to_f32)]
    pub local_offset_z2: f64,
    #[br(map = round_yew)]
    #[bw(map = to_f32)]
    pub yaw: f64,
    #[br(map = round_yew)]
    #[bw(map = to_f32)]
    pub yaw2: f64,
    #[br(little)]
    #[serde(serialize_with = "ser_item_id")]
    #[serde(deserialize_with = "de_item_id")]
    pub item_id: u16,
    #[br(little)]
    pub model_index: u16,
    #[br(little)]
    pub output_object_index: i32,
    #[br(little)]
    pub input_object_index: i32,
    pub output_to_slot: i8,
    pub input_from_slot: i8,
    pub output_from_slot: i8,
    pub input_to_slot: i8,
    pub output_offset: i8,
    pub input_offset: i8,
    #[br(little)]
    pub recipe_id: u16,
    #[br(little)]
    pub filter_id: u16,
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
