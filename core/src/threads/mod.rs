use crate::logging::Logger;
use std::sync::{Arc, Mutex};
use thread::*;

pub(crate) mod single_value_channel;

mod shared_state;
mod thread;

mod drop;
mod get;
mod kill;
mod new;
mod set_event_queue;
mod spawn;

pub(crate) use shared_state::*;
pub(crate) use single_value_channel::*;

/// Tracks all running threads on the system
pub(crate) struct ThreadManager {
    /// The state shared between all threads
    shared_state: Arc<GlobalSharedState>,

    /// The threads that have been spawned
    threads: Mutex<Vec<Thread>>,

    /// The logger the thread manager uses
    logger: Logger,

    /// A single value channel to hold panic information from threads
    panic_receiver: Mutex<Option<SingleValueReceiver<String>>>,
}
