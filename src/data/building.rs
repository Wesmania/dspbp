use serde::{Deserialize, Serialize};
use struct_deser_derive::StructDeser;

use crate::serialize::{Deser, Ser};

use super::{F32, station::Station, enums::DSPItem};

#[derive(Serialize, Deserialize, StructDeser)]
pub struct BuildingHeader {
    #[le] index: u32,
    area_index: i8,
    #[le] local_offset_x: F32,
    #[le] local_offset_y: F32,
    #[le] local_offset_z: F32,
    #[le] local_offset_x2: F32,
    #[le] local_offset_y2: F32,
    #[le] local_offset_z2: F32,
    #[le] yaw: F32,
    #[le] yaw2: F32,
    #[le] item_id: u16,
    #[le] model_index: u16,
    #[le] output_object_index: u32,
    #[le] input_object_index: u32,
    output_to_slot: i8,
    input_from_slot: i8,
    output_from_slot: i8,
    input_to_slot: i8,
    output_offset: i8,
    input_offset: i8,
    #[le] recipe_id: u16,
    #[le] filter_id: u16,
    #[le] parameter_count: u16,
}

#[derive(Serialize, Deserialize)]
pub struct Building {
    header: BuildingHeader,
    station: Option<Station>,
    params: Vec<u32>,
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
}

impl Building {
    pub fn from_bp(d: &mut Deser) -> anyhow::Result<Self> {
        let header: BuildingHeader = d.read_type()?;
        let mut station = None;
        let mut params: Vec<u32> = vec![];
        if header.has_station() {
            station = Some(Station::from_bp(d, header.has_interstellar(), header.parameter_count as usize * 4)?);
        } else {
            params.append(&mut d.skip(header.parameter_count as usize * 4)?
                                     .chunks_exact(4)
                                     .map(|b| u32::from_le_bytes(b.try_into().unwrap()))
                                     .collect());
        }
        Ok(Self {
            header,
            station,
            params,
        })
    }

    pub fn to_bp(&self, d: &mut Ser) -> anyhow::Result<()> {
        d.write_type(&self.header);
        if self.station.is_some() {
            self.station.as_ref().unwrap().to_bp(d, self.header.parameter_count as usize * 4)?;
        } else {
            let le32_vec: Vec<u8> = self.params
                .iter()
                .flat_map(|b| b.to_le_bytes().into_iter())
                .collect();
            d.append(&le32_vec);
        }
        Ok(())
    }
}
