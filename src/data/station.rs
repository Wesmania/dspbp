use std::io::{Read, Write};

#[cfg(feature = "dump")]
use serde::{Deserialize, Serialize};
use struct_deser::SerializedByteLen;
use struct_deser_derive::StructDeser;

use crate::{
    error::Error, serialize::{ReadType, WriteType}, stats::{GetStats, Stats},
};

use super::{vec::{from32le, to32le}, traits::{ReplaceItem, Replace}, enums::DSPItem};

#[cfg_attr(feature = "dump", derive(Serialize, Deserialize))]
#[derive(StructDeser)]
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

#[cfg_attr(feature = "dump", derive(Serialize, Deserialize))]
#[derive(StructDeser)]
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

#[cfg_attr(feature = "dump", derive(Serialize, Deserialize))]
#[derive(StructDeser)]
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

#[cfg_attr(feature = "dump", derive(Serialize, Deserialize))]
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

    fn storage_len(is_interstellar: bool) -> usize {
        if is_interstellar {
            5
        } else {
            3
        }
    }

    pub fn from_bp(
        mut d: &mut dyn Read,
        is_interstellar: bool,
        struct_len: usize,
    ) -> anyhow::Result<Self> {
        let slots_len = 12;
        let storage_len = Self::storage_len(is_interstellar);

        let mut storage = vec![];
        let mut slots = vec![];

        let mut read = 0;

        for _ in 0..storage_len {
            storage.push(d.read_type()?);
            read += StationStorage::BYTE_LEN;
        }

        d.skip(Self::HEADER_OFFSET - read)?;
        read = Self::HEADER_OFFSET;

        let header = d.read_type()?;
        read += StationHeader::BYTE_LEN;

        d.skip(Self::SLOTS_OFFSET - read)?;
        read = Self::SLOTS_OFFSET;
        for _ in 0..slots_len {
            slots.push(d.read_type()?);
            read += StationSlots::BYTE_LEN;
        }

        if struct_len < read {
            return Err(Error::E(format!(
                "Unexpected station data length {} at read",
                read
            ))
            .into());
        }
        let mut rest = vec![0u8; struct_len - read];
        d.read_exact(&mut rest)?;
        let unknown = to32le(&rest); // TODO might always be empty?

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

    pub fn to_bp(&self, mut s: &mut dyn Write) -> anyhow::Result<()> {
        let mut written = 0;

        for sto in &self.storage {
            s.write_type(sto)?;
            written += StationStorage::BYTE_LEN;
        }
        s.pad(Self::HEADER_OFFSET - written)?;
        written = Self::HEADER_OFFSET;

        s.write_type(&self.header)?;
        written += StationHeader::BYTE_LEN;

        s.pad(Self::SLOTS_OFFSET - written)?;
        // written = Self::SLOTS_OFFSET;

        for sl in &self.slots {
            s.write_type(sl)?;
        }

        s.write_all(&from32le(&self.unknown))?;

        Ok(())
    }
}

impl ReplaceItem for Station {
    fn replace_item(&mut self, replace: &Replace<DSPItem>) {
        for item in &mut self.storage {
            item.item_id.replace_item(replace)
        }
    }
}

impl GetStats for Station {
    fn get_stats(&self, stats: &mut Stats) {
        for s in &self.storage {
            if let Ok(b) = s.item_id.try_into() {
                stats.add_station_ware(b);
            }
        }
    }
}
