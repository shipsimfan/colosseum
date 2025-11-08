use crate::logging::Logger;
use win32::{ComPtr, d3d11sdklayers::ID3D11InfoQueue};

mod empty_queue;
mod new;

/// The info queue of messages from the system graphics API
pub(in crate::graphics::context) struct InfoQueue {
    /// A handle to the info queue
    handle: ComPtr<ID3D11InfoQueue>,

    /// The logger for printing messages
    logger: Logger,
}
