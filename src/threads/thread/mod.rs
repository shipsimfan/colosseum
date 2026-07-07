use crate::{Result, SingleValueReceiver};
use std::thread::JoinHandle;

mod join;
mod new;

/// A handle to a thread in the manager
pub(in crate::threads) struct Thread {
    /// The join handle for the thread
    join_handle: JoinHandle<()>,

    /// The result of the thread, if it has finished
    result: SingleValueReceiver<Result<()>>,

    /// The function to call to kill the thread
    on_kill: Box<dyn FnOnce()>,

    /// The name of the thread
    name: String,
}
