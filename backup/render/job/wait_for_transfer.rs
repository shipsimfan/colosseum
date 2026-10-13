use crate::{
    Result,
    render::{MeshTransfer, RenderJob},
};

impl<'surface> RenderJob<'surface> {
    /// Wait for a transfer to complete
    pub fn wait_for_transfer(&mut self, transfer: &mut MeshTransfer) -> Result<()> {
        match self {
            RenderJob::Rendering { device, .. } => device,
            RenderJob::RecreateSwapchain { device, .. } => device,
        }
        .wait_for_transfer(transfer)
    }
}
