use crate::{Result, render::LoadingMesh};

impl LoadingMesh {
    /// Wait for the transfer to complete
    pub(in crate::render) fn wait(&self) -> Result<()> {
        self.receiver.wait_no_take()
    }
}
