use binrw::{BinRead, BinWrite};
#[cfg(feature = "dump")]
use serde::{Deserialize, Serialize};

use super::{
    enums::DSPItem,
    traits::{Replace, ReplaceItem},
};

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
