use crate::render::{MeshTransfer, transfer::SharedGpuTransferData};
use std::sync::Arc;

impl MeshTransfer {
    /// Create a new [`MeshTransfer`]
    pub(in crate::render::transfer) fn new(
        shared_state: Arc<SharedGpuTransferData>,
    ) -> MeshTransfer {
        MeshTransfer { shared_state }
    }
}
