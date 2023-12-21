use crate::data::{
    building::Building,
    visit::{Visit, Visitor},
};

pub struct Round;

impl Round {
    // Round yaw to multiple of a degree.
    fn round_yaw(&self, yaw: &mut f32) {
        *yaw = (*yaw).round()
    }

    // Round loc to 1/64th of a tile.
    fn round_loc(&self, loc: &mut f32) {
        *loc = (((*loc as f64) * 64.0).round() / 64.0) as f32
    }
}

impl Visitor for Round {
    fn visit_building(&mut self, v: &mut Building) {
        let h = &mut v.header;
        self.round_loc(&mut h.local_offset_x);
        self.round_loc(&mut h.local_offset_y);
        self.round_loc(&mut h.local_offset_z);
        self.round_loc(&mut h.local_offset_x2);
        self.round_loc(&mut h.local_offset_y2);
        self.round_loc(&mut h.local_offset_z2);
        self.round_yaw(&mut h.yaw);
        self.round_yaw(&mut h.yaw2);
        v.visit(self)
    }
}
