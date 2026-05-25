use crate::Result;
use std::sync::atomic::{AtomicBool, AtomicPtr};

mod drop;
mod get;
mod new;
mod set;

/// The state shared between a single thread and the thread manager
pub(in crate::threads::thread) struct ThreadSharedState {
    /// The name of the thread
    name: String,

    /// Is the thread still running?
    is_running: AtomicBool,

    /// The result of the thread, if it has finished
    result: AtomicPtr<Result<()>>,
}
