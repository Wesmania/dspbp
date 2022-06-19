use crate::data::{visit::{Visitor, Visit}, enums::{DSPIcon, DSPItem, DSPRecipe, BPModel}, building::Building};

pub type Replace<T> = dyn Fn(T) -> T;

pub struct ReplaceItem<'a>(&'a Replace<DSPItem>);

fn ri<T: TryInto<DSPItem> + From<DSPItem> + Copy + std::fmt::Debug>(s: &Replace<DSPItem>, t: &mut T) {
    let my_item = match (*t).try_into() {
        Ok(l) => l,
        _ => {
            log::warn!("Unexpected DSP item value {:?}", *t);
            return;
        },
    };
    if my_item == s(my_item) {
        return;
    }
    *t = s(my_item).into();
}

impl<'a> ReplaceItem<'a> {
    pub fn new(f: &'a Replace<DSPItem>) -> Self {
        Self(f)
    }

    fn replace_item<T: TryInto<DSPItem> + From<DSPItem> + Copy + std::fmt::Debug>(&self, t: &mut T) {
        ri(self.0, t)
    }
}

impl<'a> Visitor for ReplaceItem<'a> {
    fn visit_blueprint(&mut self, v: &mut crate::blueprint::Blueprint) {
        for icon in v.icons.iter_mut() {
            *icon = match DSPIcon::try_from(*icon) {
                Ok(DSPIcon::Item(i)) => DSPIcon::Item((self.0)(i)).into(),
                _ => *icon,
            };
        }

        v.visit(self)
    }

    fn visit_station_storage(&mut self, v: &mut crate::data::station::StationStorage) {
        self.replace_item(&mut v.item_id);
        v.visit(self)
    }

    fn visit_station_slots(&mut self, v: &mut crate::data::station::StationSlots) {
        v.visit(self)
    }

    fn visit_belt(&mut self, v: &mut crate::data::belt::Belt) {
        self.replace_item(&mut v.label);
        v.visit(self)
    }

    fn visit_building(&mut self, v: &mut crate::data::building::Building) {
        self.replace_item(&mut v.header.filter_id);
        v.visit(self)
    }
}

pub struct ReplaceRecipe<'a>(&'a Replace<DSPRecipe>);

impl<'a> ReplaceRecipe<'a> {
    pub fn new(f: &'a Replace<DSPRecipe>) -> Self {
        Self(f)
    }

    fn replace_recipe<T: TryInto<DSPRecipe> + From<DSPRecipe> + Copy>(&self, t: &mut T) {
        let my_item = match (*t).try_into() {
            Ok(l) => l,
            _ => return,
        };
        *t = (self.0)(my_item).into();
    }
}

impl<'a> Visitor for ReplaceRecipe<'a> {
    fn visit_blueprint(&mut self, v: &mut crate::blueprint::Blueprint) {
        for icon in v.icons.iter_mut() {
            *icon = match DSPIcon::try_from(*icon) {
                Ok(DSPIcon::Recipe(i)) => DSPIcon::Recipe((self.0)(i)).into(),
                _ => *icon,
            };
        }

        v.visit(self)
    }

    fn visit_station_storage(&mut self, v: &mut crate::data::station::StationStorage) {
        self.replace_recipe(&mut v.item_id);
        v.visit(self)
    }

    fn visit_station_slots(&mut self, v: &mut crate::data::station::StationSlots) {
        v.visit(self)
    }

    fn visit_belt(&mut self, v: &mut crate::data::belt::Belt) {
        self.replace_recipe(&mut v.label);
        v.visit(self)
    }

    fn visit_building(&mut self, v: &mut crate::data::building::Building) {
        self.replace_recipe(&mut v.header.recipe_id);
        v.visit(self)
    }
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum BuildingClass {
    Assembler,
    Belt,
    Sorter,
    Other
}

impl BuildingClass {
    pub fn replacement_is_valid(i: DSPItem, o: DSPItem) -> bool {
        return Self::from(i) == Self::from(o) && Self::from(i) != Self::Other
    }
}

impl From<DSPItem> for BuildingClass {
    fn from(i: DSPItem) -> Self {
        match i {
            DSPItem::AssemblingMachineMkI => Self::Assembler,
            DSPItem::AssemblingMachineMkII => Self::Assembler,
            DSPItem::AssemblingMachineMkIII => Self::Assembler,
            DSPItem::SorterMKI => Self::Sorter,
            DSPItem::SorterMKII => Self::Sorter,
            DSPItem::SorterMKIII => Self::Sorter,
            DSPItem::ConveyorBeltMKI => Self::Belt,
            DSPItem::ConveyorBeltMKII => Self::Belt,
            DSPItem::ConveyorBeltMKIII => Self::Belt,
            _ => Self::Other,
        }
    }
}

pub struct ReplaceBuilding<'a>(&'a Replace<DSPItem>);

impl<'a> ReplaceBuilding<'a> {
    pub fn new(f: &'a Replace<DSPItem>) -> Self {
        Self(f)
    }

    fn replace_bp_model(&self, b: &mut Building, new: DSPItem) -> anyhow::Result<()> {
        let new_model = BPModel::from_building(new)?;
        b.header.model_index = new_model.into();
        Ok(())
    }

    fn replace_building(&self, b: &mut Building) {
        let my_item = match b.header.item_id.try_into() {
            Ok(l) => l,
            _ => {
                log::warn!("Unexpected DSP item value {:?}", b.header.item_id);
                return;
            },
        };
        let new_item = self.0(my_item);
        if my_item == new_item {
            return;
        }
        b.header.item_id = new_item.into();
        let _ = self.replace_bp_model(b, new_item);
    }
}

impl<'a> Visitor for ReplaceBuilding<'a> {
    fn visit_building(&mut self, v: &mut Building) {
        self.replace_building(v);
        v.visit(self)
    }
}
