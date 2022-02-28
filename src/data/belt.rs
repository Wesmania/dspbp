use std::io::Cursor;

use binrw::{BinWrite, BinRead, BinReaderExt};
#[cfg(feature = "dump")]
use serde::{Deserialize, Serialize};

use super::{traits::{ReplaceItem, Replace}, enums::DSPItem};

#[cfg_attr(feature = "dump", derive(Serialize, Deserialize))]
#[derive(BinRead, BinWrite)]
pub struct Belt {
    #[br(little)]
    label: u32,
    #[br(little)]
    count: u32,
}

impl Belt {
    pub fn from_bp(d: &mut Cursor<Vec<u8>>) -> anyhow::Result<Self> {
        d.read_le().map_err(|e| e.into())
    }

    pub fn to_bp(&self, d: &mut Cursor<Vec<u8>>) -> anyhow::Result<()> {
        self.write_to(d).map_err(|e| e.into())
    }
}

impl ReplaceItem for Belt {
    fn replace_item(&mut self, replace: &Replace<DSPItem>) {
        self.label.replace_item(replace)
    }
}
