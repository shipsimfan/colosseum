use crate::{Result, render::MeshTransfer};
use std::time::Duration;

impl MeshTransfer {
    /// Wait for the mesh transfer to complete
    pub fn wait(&self, timeout: Option<Duration>) -> Result<bool> {
        self.shared_state.wait(timeout)
    }
}
