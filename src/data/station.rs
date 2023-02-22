use std::f64::consts::PI;

use binrw::{BinRead, BinWrite};
#[cfg(feature = "dump")]
use serde::{Deserialize, Serialize};

use super::visit::{Visit, Visitor};

#[cfg_attr(feature = "dump", derive(Serialize, Deserialize))]
#[derive(BinRead, BinWrite)]
pub struct StationHeader {
    #[br(little)]
    pub work_energy_per_tick: u32, // In watts. 1M is 60MW/s.
    #[br(little)]
    pub drone_range: u32, // cos(angle) * 10^8
    #[br(little)]
    pub vessel_range: u32, // 1LY = 24000
    #[br(little)]
    pub orbital_collector: u32,
    #[br(little)]
    pub warp_distance: u32, // 1AU = 40000
    #[br(little)]
    pub equip_warper: u32,
    #[br(little)]
    pub drone_min_capacity: u32,
    #[br(little)]
    pub vessel_min_capacity: u32,
    #[br(little)]
    pub piler_count: u32,
}

impl StationHeader {
    pub const LY: usize = 24000;
    pub const AU: usize = 40000;
    pub fn angle_to_drone_range(angle: usize) -> u32 {
        let angle = f64::cos((angle as f64) / 180.0 * PI);
        (angle * 100_000_000.0).round() as u32
    }
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

impl Visit for StationSlots {
    fn visit<T: Visitor + ?Sized>(&mut self, _visitor: &mut T) {}
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

impl Visit for StationStorage {
    fn visit<T: Visitor + ?Sized>(&mut self, _visitor: &mut T) {}
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
    pub storage: Vec<StationStorage>,
    #[br(count = 192 - 30)]
    pub unknown1: Vec<u32>,

    #[br(count = 12)]
    pub slots: Vec<StationSlots>, // Counter-clockwise, from rightmost north.
    #[br(count = 320 - 192 - 48)]
    pub unknown2: Vec<u32>,

    pub header: StationHeader, // 320
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

impl Visit for Station {
    fn visit<T: Visitor + ?Sized>(&mut self, visitor: &mut T) {
        for s in self.valid_storage_mut() {
            visitor.visit_station_storage(s);
        }
        for s in self.slots.iter_mut() {
            visitor.visit_station_slots(s);
        }
    }
}

#[cfg(test)]
mod test {
    use std::io::Cursor;

    use binrw::BinWrite;

    use crate::{
        blueprint::Blueprint,
        data::{building::BuildingParam, enums::DSPItem, station::StationHeader},
        testutil::get_file,
    };

    #[test]
    fn example_station_1() {
        let bp_file = "Example interstellar station 1.txt";
        let f = get_file(bp_file);
        let (bp, raw) = Blueprint::new_with_raw_bp(std::str::from_utf8(&f).unwrap()).unwrap();
        // Drones / ships / warpers part is irrelevant, not kept in blueprint.
        let description = "Example station 1.\n\
            * Wares: blue/red/yellow/purple/green cubes.\n\
            * 3 drones, 2 ships, 1 warper.\n\
            * 60MW charging power. 50 degrees drone range. 6 ly vessel range. 2AU warp activation. \
            20% min drone load. 40% min vessel load. Orbital collector on. Warpers required on.\n\
            * Slots all out. From north leftmost clockwise: blue-red-red, \
            blue-yellow-yellow, blue-purple-purple, blue-green-green.";
        assert_eq!(&bp.get_description().unwrap(), description);

        let station = bp
            .data
            .buildings
            .iter()
            .find(|b| b.kind() == Ok(DSPItem::InterstellarLogisticsStation))
            .unwrap();
        let station = match &station.param {
            BuildingParam::Station(s) => s,
            _ => panic!(),
        };

        let sto = &station.storage;
        assert_eq!(sto[0].item_id, DSPItem::ElectromagneticMatrix as u32);
        assert_eq!(sto[1].item_id, DSPItem::EnergyMatrix as u32);
        assert_eq!(sto[2].item_id, DSPItem::StructureMatrix as u32);
        assert_eq!(sto[3].item_id, DSPItem::InformationMatrix as u32);
        assert_eq!(sto[4].item_id, DSPItem::GravityMatrix as u32);

        let h = &station.header;
        assert_eq!(h.work_energy_per_tick, 1000_000); // 1 MW per tick, 60 MW per second
        assert_eq!(h.drone_range, StationHeader::angle_to_drone_range(50));
        assert_eq!(h.vessel_range, (StationHeader::LY * 6) as u32);
        assert_eq!(h.orbital_collector, 1);
        assert_eq!(h.warp_distance, (StationHeader::AU * 2) as u32);
        assert_eq!(h.equip_warper, 1);
        assert_eq!(h.drone_min_capacity, 20);
        assert_eq!(h.vessel_min_capacity, 40);
        assert_eq!(h.piler_count, 0);

        let s = &station.slots;
        let idx: Vec<u32> = s.iter().map(|x| x.storage_index).collect();
        assert_eq!(&idx, &[2, 2, 1, 5, 5, 1, 4, 4, 1, 3, 3, 1]);

        // Can't compare whole blueprints since gzip isn't really reproducible.
        let mut back = vec![];
        bp.data.write_le(&mut Cursor::new(&mut back)).unwrap();
        assert_eq!(raw, back);
    }
}
