use crate::{
    Error, Result, debug,
    graphics::{
        GraphicsContext,
        context::{BUFFER_COUNT, RENDER_TARGET_FORMAT, SWAP_CHAIN_FLAGS, SwapchainObjects},
    },
    math::Vector2u,
};
use win32::try_hresult;

impl GraphicsContext {
    /// Resize any assets directly tied to window size
    pub(crate) fn resize(&mut self) -> Result<()> {
        let new_size = self.message_thread.window_size();
        if self.swapchain_size == new_size {
            return Ok(());
        }

        self.force_resize(new_size)
    }

    /// Forcefully resize any assets directly tied to window size
    pub(in crate::graphics::context) fn force_resize(&mut self, new_size: Vector2u) -> Result<()> {
        if let Some(swapchain_objects) = &mut self.swapchain_objects {
            swapchain_objects.unbind(&mut self.device_context);
        }
        self.swapchain_objects = None;

        try_hresult!(self.swapchain.resize_buffers(
            BUFFER_COUNT,
            new_size.x,
            new_size.y,
            RENDER_TARGET_FORMAT,
            SWAP_CHAIN_FLAGS,
        ))
        .map_err(|os| Error::new_inner("unable to resize swapcahin", os))?;

        self.swapchain_objects = Some(SwapchainObjects::new(
            &mut self.swapchain,
            new_size,
            &self.device,
        )?);

        self.swapchain_size = new_size;
        debug!(
            self.logger,
            "Swapchain resized to {}x{}", new_size.x, new_size.y
        );
        Ok(())
    }
}
