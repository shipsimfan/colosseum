use shared_state::ThreadSharedState;
use std::{sync::Arc, thread::JoinHandle};

mod shared_state;

mod join;
mod new;

/// A handle to a thread in the manager
pub(in crate::threads) struct Thread {
    /// The join handle for the thread
    join_handle: JoinHandle<()>,

    /// The state shared between the thread and the thread manager
    shared_state: Arc<ThreadSharedState>,

    /// The function to call to kill the thread
    on_kill: Box<dyn FnOnce()>,
}
