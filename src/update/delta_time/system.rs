use crate::update::DeltaTime;
use colosseum_ecs::ResMut;
use std::time::Instant;

impl DeltaTime {
    /// The system for updating the delta time each frame
    pub(in crate::update) fn system(mut self: ResMut<Self>) {
        let now = Instant::now();
        self.delta_time = now.duration_since(self.last_frame);
        self.last_frame = now;
    }
}
