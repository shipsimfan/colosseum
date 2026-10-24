use std::time::{Duration, Instant};

mod deref;
mod new;
mod system;

/// The time since the last frame
pub struct DeltaTime {
    /// The duration of the last frame
    delta_time: Duration,

    /// The time of the last frame
    last_frame: Instant,
}
