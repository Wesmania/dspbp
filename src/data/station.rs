use serde::{Deserialize, Serialize};
use struct_deser::SerializedByteLen;
use struct_deser_derive::StructDeser;

use crate::{
    error::Error,
    serialize::{Deser, Ser},
};

use super::vec::{from32le, to32le};

#[derive(Serialize, Deserialize, StructDeser)]
pub struct StationHeader {
    #[le]
    work_energy: u32,
    #[le]
    drone_range: u32,
    #[le]
    vessel_range: u32,
    #[le]
    orbital_collector: u32,
    #[le]
    warp_distance: u32,
    #[le]
    equip_warper: u32,
    #[le]
    drone_count: u32,
    #[le]
    vessel_count: u32,
}

#[derive(Serialize, Deserialize, StructDeser)]
pub struct StationSlots {
    #[le]
    direction: u32,
    #[le]
    storage_index: u32,
    #[le]
    _unused1: u32,
    #[le]
    _unused2: u32,
}

#[derive(Serialize, Deserialize, StructDeser)]
pub struct StationStorage {
    #[le]
    item_id: u32,
    #[le]
    local_logic: u32,
    #[le]
    remote_logic: u32,
    #[le]
    max_count: u32,
    #[le]
    _unused1: u32,
    #[le]
    _unused2: u32,
}

#[derive(Serialize, Deserialize)]
pub struct Station {
    header: StationHeader,
    is_interstellar: bool,
    storage: Vec<StationStorage>,
    slots: Vec<StationSlots>,
    unknown: Vec<u32>,
}

impl Station {
    const HEADER_OFFSET: usize = 128;
    const SLOTS_OFFSET: usize = 192;

    pub fn from_bp(
        d: &mut Deser,
        is_interstellar: bool,
        struct_len: usize,
    ) -> anyhow::Result<Self> {
        let slots_len = 12;
        let storage_len = if is_interstellar { 5 } else { 3 };

        let mut storage = vec![];
        let mut slots = vec![];

        let start_len = d.len();
        let header_off = start_len - Self::HEADER_OFFSET;
        let slot_off = start_len - Self::SLOTS_OFFSET;
        let end_off = start_len - struct_len;

        for _ in 0..storage_len {
            storage.push(d.read_type()?);
        }
        d.skip(d.len() - header_off)?;

        let header = d.read_type()?;

        d.skip(d.len() - slot_off)?;
        for _ in 0..slots_len {
            slots.push(d.read_type()?);
        }

        let end_len = d.len();
        if end_len < end_off {
            return Err(Error::E(format!(
                "Unexpected station data length {} at read",
                struct_len
            ))
            .into());
        }
        let unknown = to32le(d.skip(end_len - end_off)?); // TODO might always be empty?

        Ok(Self {
            header,
            is_interstellar,
            storage,
            slots,
            unknown,
        })
    }

    // In bytes
    pub fn bp_len(&self) -> usize {
        Self::SLOTS_OFFSET + 12 * StationSlots::BYTE_LEN + self.unknown.len()
    }

    pub fn to_bp(&self, s: &mut Ser) -> anyhow::Result<()> {
        let len = s.len();
        let header_off = len + Self::HEADER_OFFSET;
        let slot_off = len + Self::SLOTS_OFFSET;

        for sto in &self.storage {
            s.write_type(sto);
        }
        s.pad(header_off - s.len());

        s.write_type(&self.header);
        s.pad(slot_off - s.len());

        for sl in &self.slots {
            s.write_type(sl);
        }

        s.append(&from32le(&self.unknown));

        Ok(())
    }
}
