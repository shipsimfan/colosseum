use std::sync::atomic::AtomicBool;

mod is_running;
mod kill;
mod new;

/// Tracks if the engine should be running or not
pub(crate) struct RunningState {
    /// Is the engine currently running?
    is_running: AtomicBool,
}
