use serde::{Deserialize, Serialize};
use struct_deser_derive::StructDeser;

use crate::serialize::{Deser, Ser};

use super::{
    belt::Belt,
    enums::DSPItem,
    station::Station,
    vec::{from32le, to32le},
    F32,
};

#[derive(Serialize, Deserialize)]
pub enum BuildingParam {
    Station(Station),
    Belt(Option<Belt>),
    Unknown(Vec<u32>),
}

impl BuildingParam {
    pub fn from_bp(header: &BuildingHeader, d: &mut Deser) -> anyhow::Result<Self> {
        if header.has_station() {
            let station = Station::from_bp(
                d,
                header.has_interstellar(),
                header.parameter_count as usize * 4,
            )?;
            Ok(BuildingParam::Station(station))
        } else if header.is_belt() {
            let belt = if header.parameter_count > 0 {
                Some(Belt::from_bp(d)?)
            } else {
                None
            };
            Ok(BuildingParam::Belt(belt))
        } else {
            let params: Vec<u32> = to32le(d.skip(header.parameter_count as usize * 4)?);
            Ok(BuildingParam::Unknown(params))
        }
    }

    pub fn bp_len(&self) -> usize {
        match self {
            Self::Station(s) => s.bp_len(),
            Self::Belt(Some(b)) => b.bp_len(),
            Self::Belt(None) => 0,
            Self::Unknown(v) => v.len() * 4,
        }
    }

    pub fn to_bp(&self, d: &mut Ser) -> anyhow::Result<()> {
        match self {
            Self::Station(s) => s.to_bp(d),
            Self::Belt(Some(b)) => Ok(b.to_bp(d)),
            Self::Belt(None) => Ok(()),
            Self::Unknown(v) => {
                d.append(&from32le(v));
                Ok(())
            }
        }
    }
}

#[derive(Serialize, Deserialize, StructDeser)]
pub struct BuildingHeader {
    #[le]
    index: u32,
    area_index: i8,
    #[le]
    local_offset_x: F32,
    #[le]
    local_offset_y: F32,
    #[le]
    local_offset_z: F32,
    #[le]
    local_offset_x2: F32,
    #[le]
    local_offset_y2: F32,
    #[le]
    local_offset_z2: F32,
    #[le]
    yaw: F32,
    #[le]
    yaw2: F32,
    #[le]
    item_id: u16,
    #[le]
    model_index: u16,
    #[le]
    output_object_index: u32,
    #[le]
    input_object_index: u32,
    output_to_slot: i8,
    input_from_slot: i8,
    output_from_slot: i8,
    input_to_slot: i8,
    output_offset: i8,
    input_offset: i8,
    #[le]
    recipe_id: u16,
    #[le]
    filter_id: u16,
    #[le]
    parameter_count: u16,
}

#[derive(Serialize, Deserialize)]
pub struct Building {
    header: BuildingHeader,
    param: BuildingParam,
}

impl BuildingHeader {
    fn has_station(&self) -> bool {
        match DSPItem::try_from(self.item_id) {
            Ok(DSPItem::PlanetaryLogisticsStation) => true,
            Ok(DSPItem::InterstellarLogisticsStation) => true,
            _ => false,
        }
    }
    fn has_interstellar(&self) -> bool {
        match DSPItem::try_from(self.item_id) {
            Ok(DSPItem::InterstellarLogisticsStation) => true,
            _ => false,
        }
    }

    fn is_belt(&self) -> bool {
        let belts = [
            DSPItem::ConveyorBeltMKI,
            DSPItem::ConveyorBeltMKII,
            DSPItem::ConveyorBeltMKIII,
        ];
        DSPItem::try_from(self.item_id).map_or(false, |i| belts.contains(&i))
    }
}

impl Building {
    pub fn from_bp(d: &mut Deser) -> anyhow::Result<Self> {
        let header: BuildingHeader = d.read_type()?;
        let param = BuildingParam::from_bp(&header, d)?;
        Ok(Self { header, param })
    }

    pub fn to_bp(&self, d: &mut Ser) -> anyhow::Result<()> {
        d.write_type(&self.header);
        self.param.to_bp(d)
    }
}
