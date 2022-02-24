use serde::{Deserialize, Serialize};
use struct_deser_derive::StructDeser;

use crate::serialize::{Deser, Ser};

use super::{traits::{ReplaceItem, Replace}, enums::DSPItem};

#[derive(Serialize, Deserialize, StructDeser)]
pub struct Belt {
    #[le]
    label: u32,
    #[le]
    count: u32,
}

impl Belt {
    pub fn from_bp(d: &mut Deser) -> anyhow::Result<Self> {
        d.read_type().map_err(|e| e.into())
    }

    pub fn bp_len(&self) -> usize {
        8
    }

    pub fn to_bp(&self, d: &mut Ser) {
        d.write_type(self)
    }
}

impl ReplaceItem for Belt {
    fn replace_item(&mut self, replace: &Replace<DSPItem>) {
        self.label.replace_item(replace)
    }
}
