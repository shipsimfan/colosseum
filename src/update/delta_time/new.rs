use crate::update::DeltaTime;
use std::time::{Duration, Instant};

impl DeltaTime {
    /// Create a new [`DeltaTime`] instance
    pub(in crate::update) fn new() -> DeltaTime {
        DeltaTime {
            delta_time: Duration::ZERO,
            last_frame: Instant::now(),
        }
    }
}
