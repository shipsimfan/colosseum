use crate::render::transfer::SharedGpuTransferData;
use std::sync::atomic::Ordering;

impl SharedGpuTransferData {
    /// Is the transfer complete?
    pub fn is_complete(&self) -> bool {
        self.is_complete.load(Ordering::Acquire)
    }
}
