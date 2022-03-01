use binrw::{BinWrite, BinRead};
#[cfg(feature = "dump")]
use serde::{Deserialize, Serialize};

use crate::stats::{GetStats, Stats};

use super::{traits::{ReplaceItem, Replace}, enums::DSPItem};

#[cfg_attr(feature = "dump", derive(Serialize, Deserialize))]
#[derive(BinRead, BinWrite)]
pub struct StationHeader {
    #[br(little)]
    work_energy_per_tick: u32,
    #[br(little)]
    drone_range: u32,
    #[br(little)]
    vessel_range: u32,
    #[br(little)]
    orbital_collector: u32,
    #[br(little)]
    warp_distance: u32,
    #[br(little)]
    equip_warper: u32,
    #[br(little)]
    drone_min_capacity: u32,
    #[br(little)]
    vessel_min_capacity: u32,
    #[br(little)]
    piler_count: u32,
}

#[cfg_attr(feature = "dump", derive(Serialize, Deserialize))]
#[derive(BinRead, BinWrite)]
pub struct StationSlots {
    #[br(little)]
    direction: u32,
    #[br(little)]
    storage_index: u32,
    #[br(little)]
    unused1: u32,
    #[br(little)]
    unused2: u32,
}

#[cfg_attr(feature = "dump", derive(Serialize, Deserialize))]
#[derive(BinRead, BinWrite)]
pub struct StationStorage {
    #[br(little)]
    item_id: u32,
    #[br(little)]
    local_logic: u32,
    #[br(little)]
    remote_logic: u32,
    #[br(little)]
    max_count: u32,
    #[br(little)]
    unused1: u32,
    #[br(little)]
    unused2: u32,
}

#[cfg_attr(feature = "dump", derive(Serialize, Deserialize))]
#[derive(BinRead, BinWrite)]
#[br(pre_assert(param_count == 2048))]
#[br(import { is_interstellar: bool, param_count: usize })]
pub struct Station {
    #[br(calc = is_interstellar)]
    #[bw(ignore)]
    is_interstellar: bool,

    // ignore last 2 if not interstellar
    #[br(count = 5)]
    storage: Vec<StationStorage>,           // 0 (in &u32)
    #[br(count = 192 - 30)]
    unknown1: Vec<u32>,

    #[br(count = 12)]
    slots: Vec<StationSlots>,               // 192
    #[br(count = 320 - 192 - 48)]
    unknown2: Vec<u32>,

    header: StationHeader,                  // 320
    #[br(count = 2048 - 320 - 9)]
    unknown3: Vec<u32>,
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
