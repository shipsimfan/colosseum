use crate::{
    Result,
    render::{MeshTransfer, job::GraphicsDevice},
};

impl GraphicsDevice {
    /// Wait for a transfer to complete
    pub fn wait_for_transfer(&mut self, transfer: &mut MeshTransfer) -> Result<()> {
        match &mut self.gpu_transfer_queue {
            Some(queue) => {
                while queue.handle_command(&mut self.queue, false)? {}
                Ok(())
            }
            None => transfer.wait(),
        }
    }
}
