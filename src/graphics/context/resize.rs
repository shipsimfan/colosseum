use crate::{
    Error, ManagedObjects, Result, debug,
    graphics::{
        GraphicsContext,
        context::{BUFFER_COUNT, SWAP_CHAIN_FLAGS, SWAPCHAIN_FORMAT, SwapchainObjects},
    },
    math::Vector2u,
};
use win32::try_hresult;

impl GraphicsContext {
    /// Resize any assets directly tied to window size
    pub(crate) fn resize(&mut self, managed_objects: &mut ManagedObjects) -> Result<()> {
        let new_size = self.message_thread.window_size();
        if self.size == new_size {
            return Ok(());
        }

        self.force_resize(managed_objects, new_size)
    }

    /// Forcefully resize any assets directly tied to window size
    pub(in crate::graphics::context) fn force_resize(
        &mut self,
        managed_objects: &mut ManagedObjects,
        new_size: Vector2u,
    ) -> Result<()> {
        // Unbind and drop the old swapchain and render scale objects
        if let Some(swapchain_objects) = &mut self.swapchain_objects {
            swapchain_objects.unbind(&mut self.device_context);
        }
        self.swapchain_objects = None;

        // Resize swapchain
        try_hresult!(self.swapchain.resize_buffers(
            BUFFER_COUNT,
            new_size.x,
            new_size.y,
            SWAPCHAIN_FORMAT,
            SWAP_CHAIN_FLAGS,
        ))
        .map_err(|os| Error::new_inner("unable to resize swapcahin", os))?;

        // Recreate the swapchain objects
        self.swapchain_objects = Some(SwapchainObjects::new(
            &mut self.swapchain,
            new_size,
            &self.device,
        )?);

        // Recreate render scale objects
        self.post_processing.resize(new_size, &self.device);

        // Update the camera sizes
        for camera in &mut managed_objects.graphics.cameras {
            camera.resize();
        }

        self.size = new_size;
        debug!(
            self.logger,
            "Swapchain resized to {}x{}", new_size.x, new_size.y
        );
        Ok(())
    }
}
