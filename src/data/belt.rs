use binrw::{BinWrite, BinRead};
#[cfg(feature = "dump")]
use serde::{Deserialize, Serialize};

use super::{traits::{ReplaceItem, Replace}, enums::DSPItem};

#[cfg_attr(feature = "dump", derive(Serialize, Deserialize))]
#[derive(BinRead, BinWrite)]
#[br(import(param_count: usize))]
#[br(pre_assert(param_count == 2))]
pub struct Belt {
    #[br(little)]
    label: u32,
    #[br(little)]
    count: u32,
}

impl ReplaceItem for Belt {
    fn replace_item(&mut self, replace: &Replace<DSPItem>) {
        self.label.replace_item(replace)
    }
}
