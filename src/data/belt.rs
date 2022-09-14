use binrw::{BinRead, BinWrite};
#[cfg(feature = "dump")]
use serde::{Deserialize, Serialize};

use super::visit::Visit;

#[cfg_attr(feature = "dump", derive(Serialize, Deserialize))]
#[derive(BinRead, BinWrite)]
#[br(import(param_count: usize))]
#[br(pre_assert(param_count == 2))]
pub struct Belt {
    #[br(little)]
    pub label: u32,
    #[br(little)]
    pub count: u32,
}

impl Visit for Belt {
    fn visit<T: super::visit::Visitor + ?Sized>(&mut self, _visitor: &mut T) {}
}
