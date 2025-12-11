use crate::graphics::context::FrameCb;

impl FrameCb {
    /// Create a new [`FrameCb`]
    pub fn new() -> Self {
        FrameCb {
            frame: 0,
            time: 0.0,
            delta_time: 0.0,
            reserved: 0,
        }
    }
}
