use crate::{
    Result,
    render::{GraphicsDevice, RenderCommandBuffer},
};
use alexandria::{
    Id,
    gpu::{
        VulkanCommandBuffer, VulkanCommandBufferLevel, VulkanCommandPool,
        VulkanCommandPoolCreateFlag,
    },
};
use colosseum_core::new_error;
#[cfg(debug_assertions)]
use std::ffi::CString;

impl RenderCommandBuffer {
    /// Create a new set of [`RenderCommandBuffer`]s
    pub fn new(device: &GraphicsDevice) -> Result<RenderCommandBuffer> {
        let mut command_pool = device
            .create_command_pool(
                device.render_queue_family(),
                VulkanCommandPoolCreateFlag::ResetCommandBuffer,
            )
            .map_err(|error| new_error!("unable to create the render command pool - {}", error))?;
        #[cfg(debug_assertions)]
        device
            .set_object_name(&mut command_pool, c"Render Command Pool")
            .map_err(|error| new_error!("unable to set render command pool name - {}", error))?;

        let command_buffers = [
            allocate_command_buffer(0, &mut command_pool, device)?,
            allocate_command_buffer(1, &mut command_pool, device)?,
            allocate_command_buffer(2, &mut command_pool, device)?,
        ];

        Ok(RenderCommandBuffer {
            command_pool,
            command_buffers,
        })
    }
}

/// Allocate a single command buffer from the given command pool
fn allocate_command_buffer(
    index: usize,
    command_pool: &mut VulkanCommandPool,
    device: &GraphicsDevice,
) -> Result<Id<VulkanCommandBuffer>> {
    let command_buffer = command_pool
        .allocate_command_buffer(VulkanCommandBufferLevel::Primary)
        .map_err(|error| new_error!("unable to allocate a render command buffer - {}", error))?;

    #[cfg(debug_assertions)]
    device
        .set_object_name(
            &mut command_pool[command_buffer],
            &CString::new(format!("Render Command Buffer {}", index)).unwrap(),
        )
        .map_err(|error| {
            new_error!(
                "unable to set object name for render command buffer - {}",
                error
            )
        })?;

    Ok(command_buffer)
}
