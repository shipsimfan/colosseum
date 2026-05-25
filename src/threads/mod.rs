use std::sync::Arc;
use thread::Thread;

mod shared_state;
mod thread;

mod kill;
mod new;
mod spawn;

pub(crate) use shared_state::GlobalSharedState;

/// Tracks all running threads on the system
pub struct ThreadManager {
    /// The state shared between all threads
    shared_state: Arc<GlobalSharedState>,

    /// The threads that have been spawned
    threads: Vec<Thread>,
}
