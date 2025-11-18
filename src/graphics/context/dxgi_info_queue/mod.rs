use crate::logging::Logger;
use win32::{ComPtr, dxgidebug::IDXGIInfoQueue};

mod drop;
mod empty_queue;
mod new;

/// The info queue of messages from the system graphics API
pub(in crate::graphics::context) struct DXGIInfoQueue {
    /// A handle to the info queue
    handle: ComPtr<IDXGIInfoQueue>,

    /// The logger for printing messages
    logger: Logger,
}
