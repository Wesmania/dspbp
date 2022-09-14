use crate::blueprint::Blueprint;

use super::{
    belt::Belt,
    blueprint::BlueprintData,
    building::Building,
    station::{Station, StationSlots, StationStorage},
};

pub trait Visitor {
    fn visit_blueprint(&mut self, v: &mut Blueprint) {
        v.visit(self)
    }

    fn visit_blueprint_data(&mut self, v: &mut BlueprintData) {
        v.visit(self)
    }

    fn visit_building(&mut self, v: &mut Building) {
        v.visit(self)
    }

    fn visit_station(&mut self, v: &mut Station) {
        v.visit(self)
    }

    fn visit_station_storage(&mut self, v: &mut StationStorage) {
        v.visit(self)
    }

    fn visit_station_slots(&mut self, v: &mut StationSlots) {
        v.visit(self)
    }

    fn visit_belt(&mut self, v: &mut Belt) {
        v.visit(self)
    }
}

pub trait Visit {
    fn visit<T: Visitor + ?Sized>(&mut self, visitor: &mut T);
}
