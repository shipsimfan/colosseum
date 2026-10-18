use crate::{
    Result,
    render::{GraphicsDevice, SecondaryCommandBuffer},
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
use std::marker::PhantomData;

impl<Node> SecondaryCommandBuffer<Node> {
    /// Create a new set of [`SecondaryCommandBuffer`]s
    pub fn new(name: &str, device: &GraphicsDevice) -> Result<SecondaryCommandBuffer<Node>> {
        let mut command_pool = device
            .create_command_pool(
                device.render_queue_family(),
                VulkanCommandPoolCreateFlag::ResetCommandBuffer,
            )
            .map_err(|error| {
                new_error!(
                    "unable to create a secondary command pool for \"{}\" - {}",
                    name,
                    error
                )
            })?;

        #[cfg(debug_assertions)]
        let pool_name = CString::new(format!("Secondary Command Pool \"{}\"", name)).unwrap();
        #[cfg(debug_assertions)]
        device
            .set_object_name(&mut command_pool, &pool_name)
            .map_err(|error| {
                new_error!(
                    "unable to set secondary command pool name for \"{}\" - {}",
                    name,
                    error
                )
            })?;

        let command_buffers = [
            allocate_command_buffer(0, name, &mut command_pool, device)?,
            allocate_command_buffer(1, name, &mut command_pool, device)?,
            allocate_command_buffer(2, name, &mut command_pool, device)?,
        ];

        Ok(SecondaryCommandBuffer {
            command_pool,
            command_buffers,
            _node: PhantomData,
        })
    }
}

/// Allocate a single command buffer from the given command pool
fn allocate_command_buffer(
    index: usize,
    name: &str,
    command_pool: &mut VulkanCommandPool,
    device: &GraphicsDevice,
) -> Result<Id<VulkanCommandBuffer>> {
    let command_buffer = command_pool
        .allocate_command_buffer(VulkanCommandBufferLevel::Secondary)
        .map_err(|error| {
            new_error!(
                "unable to allocate a secondary command buffer for \"{}\" - {}",
                name,
                error
            )
        })?;

    #[cfg(debug_assertions)]
    device
        .set_object_name(
            &mut command_pool[command_buffer],
            &CString::new(format!("Secondary Command Buffer \"{}\" {}", name, index)).unwrap(),
        )
        .map_err(|error| {
            new_error!(
                "unable to set object name for secondary command buffer for \"{}\" - {}",
                name,
                error
            )
        })?;

    Ok(command_buffer)
}
