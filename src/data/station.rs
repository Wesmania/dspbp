use std::io::{Cursor, Write};

use crate::{serialize::{ReadType, WriteType}, ReadPlusSeek};

use binrw::{BinWrite, BinRead, BinReaderExt};
#[cfg(feature = "dump")]
use serde::{Deserialize, Serialize};

use crate::{
    error::Error, stats::{GetStats, Stats},
};

use super::{vec::{from32le, to32le}, traits::{ReplaceItem, Replace}, enums::DSPItem};

#[cfg_attr(feature = "dump", derive(Serialize, Deserialize))]
#[derive(BinRead, BinWrite)]
pub struct StationHeader {
    #[br(little)]
    work_energy: u32,
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
    drone_count: u32,
    #[br(little)]
    vessel_count: u32,
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
        mut d: &mut dyn ReadPlusSeek,
        is_interstellar: bool,
        struct_len: usize,
    ) -> anyhow::Result<Self> {
        let slots_len = 12;
        let storage_len = Self::storage_len(is_interstellar);

        let mut storage = vec![];
        let mut slots = vec![];

        let mut read = 0;

        for _ in 0..storage_len {
            storage.push(d.read_le()?);
            read += 24;
        }

        d.skip(Self::HEADER_OFFSET - read)?;
        read = Self::HEADER_OFFSET;

        let header = d.read_le()?;
        read += 32;

        d.skip(Self::SLOTS_OFFSET - read)?;
        read = Self::SLOTS_OFFSET;
        for _ in 0..slots_len {
            slots.push(d.read_le()?);
            read += 16;
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
        Self::SLOTS_OFFSET + 12 * 16 + self.unknown.len()
    }

    pub fn to_bp(&self, s: &mut Cursor<Vec<u8>>) -> anyhow::Result<()> {
        let mut written = 0;

        for sto in &self.storage {
            sto.write_to(s)?;
            written += 24;
        }
        s.pad(Self::HEADER_OFFSET - written)?;
        written = Self::HEADER_OFFSET;

        self.header.write_to(s)?;
        written += 32;

        s.pad(Self::SLOTS_OFFSET - written)?;
        // written = Self::SLOTS_OFFSET;

        for sl in &self.slots {
            sl.write_to(s)?;
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
