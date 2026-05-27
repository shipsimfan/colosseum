use std::sync::{Arc, Mutex};
use thread::Thread;

mod shared_state;
mod thread;

mod drop;
mod get;
mod kill;
mod new;
mod set_event_queue;
mod spawn;

pub(crate) use shared_state::GlobalSharedState;

/// Tracks all running threads on the system
pub(crate) struct ThreadManager {
    /// The state shared between all threads
    shared_state: Arc<GlobalSharedState>,

    /// The threads that have been spawned
    threads: Mutex<Vec<Thread>>,
}
