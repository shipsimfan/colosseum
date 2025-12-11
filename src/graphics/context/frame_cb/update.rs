use crate::graphics::context::FrameCb;

impl FrameCb {
    /// Update the contents of the frame buffer
    pub fn update(&mut self, delta_time: f32) {
        self.frame += 1;
        self.time += delta_time;
        self.delta_time = delta_time;
    }
}
