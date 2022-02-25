use std::io::{Read, Write};

use serde::{Deserialize, Serialize};
use struct_deser_derive::StructDeser;

use crate::serialize::{ReadType, WriteType};

use super::{traits::{ReplaceItem, Replace}, enums::DSPItem};

#[derive(Serialize, Deserialize, StructDeser)]
pub struct Belt {
    #[le]
    label: u32,
    #[le]
    count: u32,
}

impl Belt {
    pub fn from_bp(mut d: &mut dyn Read) -> anyhow::Result<Self> {
        d.read_type().map_err(|e| e.into())
    }

    pub fn bp_len(&self) -> usize {
        8
    }

    pub fn to_bp(&self, mut d: &mut dyn Write) -> anyhow::Result<()> {
        d.write_type(self)
    }
}

impl ReplaceItem for Belt {
    fn replace_item(&mut self, replace: &Replace<DSPItem>) {
        self.label.replace_item(replace)
    }
}
