use binrw::{BinWrite, BinRead};
#[cfg(feature = "dump")]
use serde::{Deserialize, Serialize};

use crate::stats::{GetStats, Stats};

use super::{traits::{ReplaceItem, Replace}, enums::DSPItem};

#[cfg_attr(feature = "dump", derive(Serialize, Deserialize))]
#[derive(BinRead, BinWrite)]
pub struct StationHeader {
    #[br(little)]
    pub work_energy_per_tick: u32,
    #[br(little)]
    pub drone_range: u32,
    #[br(little)]
    pub vessel_range: u32,
    #[br(little)]
    pub orbital_collector: u32,
    #[br(little)]
    pub warp_distance: u32,
    #[br(little)]
    pub equip_warper: u32,
    #[br(little)]
    pub drone_min_capacity: u32,
    #[br(little)]
    pub vessel_min_capacity: u32,
    #[br(little)]
    pub piler_count: u32,
}

#[cfg_attr(feature = "dump", derive(Serialize, Deserialize))]
#[derive(BinRead, BinWrite)]
pub struct StationSlots {
    #[br(little)]
    pub direction: u32,
    #[br(little)]
    pub storage_index: u32,
    #[br(little)]
    pub unused1: u32,
    #[br(little)]
    pub unused2: u32,
}

#[cfg_attr(feature = "dump", derive(Serialize, Deserialize))]
#[derive(BinRead, BinWrite)]
pub struct StationStorage {
    #[br(little)]
    pub item_id: u32,
    #[br(little)]
    pub local_logic: u32,
    #[br(little)]
    pub remote_logic: u32,
    #[br(little)]
    pub max_count: u32,
    #[br(little)]
    pub unused1: u32,
    #[br(little)]
    pub unused2: u32,
}

#[cfg_attr(feature = "dump", derive(Serialize, Deserialize))]
#[derive(BinRead, BinWrite)]
#[br(pre_assert(param_count == 2048))]
#[br(import { is_interstellar: bool, param_count: usize })]
pub struct Station {
    #[br(calc = is_interstellar)]
    #[bw(ignore)]
    pub is_interstellar: bool,

    // ignore last 2 if not interstellar
    #[br(count = 5)]
    pub storage: Vec<StationStorage>,           // 0 (in &u32)
    #[br(count = 192 - 30)]
    pub unknown1: Vec<u32>,

    #[br(count = 12)]
    pub slots: Vec<StationSlots>,               // 192
    #[br(count = 320 - 192 - 48)]
    pub unknown2: Vec<u32>,

    pub header: StationHeader,                  // 320
    #[br(count = 2048 - 320 - 9)]
    pub unknown3: Vec<u32>,
}

impl Station {
    fn storage_count(&self) -> usize {
        if self.is_interstellar {
            5
        } else {
            3
        }
    }

    fn valid_storage(&self) -> &[StationStorage] {
        let count = self.storage_count();
        &self.storage[0..count]
    }

    fn valid_storage_mut(&mut self) -> &mut [StationStorage] {
        let count = self.storage_count();
        &mut self.storage[0..count]
    }
}

impl ReplaceItem for Station {
    fn replace_item(&mut self, replace: &Replace<DSPItem>) {
        for item in self.valid_storage_mut() {
            item.item_id.replace_item(replace)
        }
    }
}

impl GetStats for Station {
    fn get_stats(&self, stats: &mut Stats) {
        for s in self.valid_storage() {
            if let Ok(b) = s.item_id.try_into() {
                stats.add_station_ware(b);
            }
        }
    }
}

#[cfg(test)]
mod test {
    use crate::{testutil::get_file, blueprint::Blueprint, data::{enums::DSPItem, building::BuildingParam}};

    #[test]
    fn example_station_1() {
        let bp_file = "Example interstellar station 1.txt";
        let f = get_file(bp_file);
        let bp = Blueprint::new(std::str::from_utf8(&f).unwrap()).unwrap();
        let description =
            "Example station 1.\n\
            * Wares: blue/red/yellow/purple/green cubes.\n\
            * 3 drones, 2 ships, 1 warper.\n\
            * 60MW charging power. 50 degrees drone range. 6 ly vessel range. 2AU warp activation. \
            20% min drone load. 40% min vessel load. Orbital collector on. Warpers required on.\n\
            * Slots all out. From north leftmost clockwise: blue-red-red, \
            blue-yellow-yellow, blue-purple-purple, blue-green-green.";
        assert!(&bp.get_description().unwrap() == description);

        let station = bp.data.buildings.iter()
            .find(|b| b.kind() == Ok(DSPItem::InterstellarLogisticsStation))
            .unwrap();
        let station = match &station.param {
            BuildingParam::Station(s) => s,
            _ => panic!(),
        };

        let sto = &station.storage;
        assert!(sto[0].item_id == DSPItem::ElectromagneticMatrix as u32);
        assert!(sto[1].item_id == DSPItem::EnergyMatrix as u32);
        assert!(sto[2].item_id == DSPItem::StructureMatrix as u32);
        assert!(sto[3].item_id == DSPItem::InformationMatrix as u32);
        assert!(sto[4].item_id == DSPItem::GravityMatrix as u32);
    }
}
