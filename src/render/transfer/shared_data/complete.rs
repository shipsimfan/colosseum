use crate::{Error, Result, render::transfer::SharedGpuTransferData};
use std::sync::atomic::Ordering;

impl SharedGpuTransferData {
    /// Mark the transfer as complete and notify any waiters
    pub fn complete(&self) -> Result<()> {
        self.is_complete.store(true, Ordering::Release);
        self.notify.notify().map_err(Error::new_inner)
    }
}
