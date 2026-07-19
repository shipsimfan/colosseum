use crate::{Error, Result, render::transfer::SharedGpuTransferData};
use alexandria::Notify;
use std::sync::{Arc, atomic::AtomicBool};

impl SharedGpuTransferData {
    /// Create a new [`SharedGpuTransferData`]
    pub fn new() -> Result<Arc<SharedGpuTransferData>> {
        Ok(Arc::new(SharedGpuTransferData {
            is_complete: AtomicBool::new(false),
            notify: Notify::new(false, false).map_err(Error::new_inner)?,
        }))
    }
}
