use crate::logging::Logger;
use alexandria::EventQueue;
use std::sync::{Mutex, atomic::AtomicBool};

mod get;
mod kill;
mod new;
mod set_event_queue;

/// The state shared between all threads
pub(crate) struct GlobalSharedState {
    /// Should the application continue running?
    is_running: AtomicBool,

    /// The logger for thread operations
    logger: Logger,

    /// The event queue to push a quit event to
    event_queue: Mutex<Option<EventQueue<()>>>,
}
