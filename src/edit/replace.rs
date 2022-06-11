use crate::data::{visit::{Visitor, Visit}, enums::{DSPIcon, DSPItem, DSPRecipe}};

pub type Replace<T> = dyn Fn(T) -> T;

pub struct ReplaceItem<'a>(&'a Replace<DSPItem>);

impl<'a> ReplaceItem<'a> {
    pub fn new(f: &'a Replace<DSPItem>) -> Self {
        Self(f)
    }

    fn replace_item<T: TryInto<DSPItem> + From<DSPItem> + Copy>(&self, t: &mut T) {
        let my_item = match (*t).try_into() {
            Ok(l) => l,
            _ => return,
        };
        *t = (self.0)(my_item).into();
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
