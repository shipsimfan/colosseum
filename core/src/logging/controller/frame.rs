use crate::LogController;
use std::sync::atomic::Ordering;

impl LogController {
    /// Increase the frame count
    pub fn frame(&self) {
        self.frame.fetch_add(1, Ordering::Acquire);
    }
}
