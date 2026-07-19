use alexandria::Notify;
use std::sync::atomic::AtomicBool;

mod complete;
mod is_complete;
mod new;
mod wait;

/// The shared state for a GPU transfer
pub(in crate::render::transfer) struct SharedGpuTransferData {
    /// Has the transfer been completed?
    is_complete: AtomicBool,

    /// The notify for when the transfer is complete
    notify: Notify,
}
