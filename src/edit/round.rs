use crate::data::{
    building::Building,
    visit::{Visit, Visitor},
};

pub struct Round;

impl Round {
    // Round yaw to a right angle if we're off by less than 3 degrees.
    fn round_yaw(&self, yaw: &mut f32) {
        let rounded: f32 = ((*yaw as f64 / 90.0).round() * 90.0) as f32;
        if (rounded - *yaw).abs() <= 3.0 {
            *yaw = rounded;
        }
    }

    // Round loc to closest half a tile if we're off by less than 1/64th.
    fn round_loc(&self, loc: &mut f32) {
        let rounded: f32 = (*loc * 2.0).round() / 2.0;
        if (rounded - *loc).abs() <= 1.0 / 64.0 {
            *loc = rounded;
        }
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
