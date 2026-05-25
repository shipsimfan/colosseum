use crate::logging::LogController;
use std::sync::atomic::Ordering;

impl LogController {
    /// Increase the frame count
    pub(crate) fn frame(&self) {
        self.frame.fetch_add(1, Ordering::Acquire);
    }
}
