use crate::logging::Logger;
use std::sync::atomic::AtomicBool;

mod get;
mod new;
mod set;

/// The state shared between all threads
pub(crate) struct GlobalSharedState {
    /// Should the application continue running?
    is_running: AtomicBool,

    /// The logger for thread operations
    logger: Logger,
}
