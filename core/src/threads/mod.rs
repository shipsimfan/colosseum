use crate::{Logger, SingleValueReceiver};
use std::sync::{Arc, Mutex};
use thread::*;

mod shared_state;
mod thread;

mod drop;
mod get;
mod kill;
mod new;
mod set_event_queue;
mod spawn;

pub use shared_state::*;

/// Tracks all running threads on the system
pub struct ThreadManager<UserEvent: 'static + Send> {
    /// The state shared between all threads
    shared_state: Arc<GlobalSharedState<UserEvent>>,

    /// The threads that have been spawned
    threads: Mutex<Vec<Thread>>,

    /// The logger the thread manager uses
    logger: Logger,

    /// A single value channel to hold panic information from threads
    panic_receiver: Mutex<Option<SingleValueReceiver<String>>>,
}
