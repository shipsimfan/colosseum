use crate::{Error, Result, render::transfer::SharedGpuTransferData};
use std::time::Duration;

impl SharedGpuTransferData {
    /// Wait for the transfer to complete
    pub fn wait(&self, timeout: Option<Duration>) -> Result<bool> {
        self.notify.wait(timeout).map_err(Error::new_inner)
    }
}
