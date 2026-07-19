use crate::render::transfer::SharedGpuTransferData;
use std::sync::Arc;

mod is_complete;
mod new;
mod wait;

/// The state of a mesh transfer
pub struct MeshTransfer {
    /// The shared state for the transfer
    shared_state: Arc<SharedGpuTransferData>,
}
