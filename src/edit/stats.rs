use crate::{
    data::visit::{Visit, Visitor},
    stats::Stats,
};

pub struct GetStats(pub Stats);

impl GetStats {
    pub fn new() -> Self {
        Self(Stats::new())
    }
}

impl Visitor for GetStats {
    fn visit_building(&mut self, v: &mut crate::data::building::Building) {
        if let Ok(b) = v.header.item_id.try_into() {
            self.0.add_building(b);
        }
        if let Ok(b) = v.header.recipe_id.try_into() {
            self.0.add_recipe(b);
        }
        v.visit(self)
    }

    fn visit_station_storage(&mut self, v: &mut crate::data::station::StationStorage) {
        if let Ok(b) = v.item_id.try_into() {
            self.0.add_station_ware(b);
        }
        v.visit(self)
    }
}
