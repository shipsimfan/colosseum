use crate::{
    Error, Result,
    render::{
        FrameGraphTransientBuffer,
        job::{GraphicsDevice, swapchain::FrameData},
    },
};
use alexandria::gpu::{VulkanCommandBufferLevel, VulkanCommandPool, VulkanFenceCreateFlag};
#[cfg(debug_assertions)]
use std::ffi::CString;

impl FrameData {
    /// Creates a new [`FrameData`]
    pub(in crate::render::job::swapchain) fn new(
        index: usize,
        command_pool: &mut VulkanCommandPool,
        device: &GraphicsDevice,
    ) -> Result<FrameData> {
        let copy_command_buffer = command_pool
            .allocate_command_buffer(VulkanCommandBufferLevel::Primary)
            .map_err(Error::new_inner)?;
        #[cfg(debug_assertions)]
        device
            .set_object_name(
                &mut command_pool[copy_command_buffer],
                &CString::new(format!("Copy Command Buffer {}", index)).unwrap(),
            )
            .map_err(Error::new_inner)?;

        let render_command_buffer = command_pool
            .allocate_command_buffer(VulkanCommandBufferLevel::Primary)
            .map_err(Error::new_inner)?;
        #[cfg(debug_assertions)]
        device
            .set_object_name(
                &mut command_pool[render_command_buffer],
                &CString::new(format!("Render Command Buffer {}", index)).unwrap(),
            )
            .map_err(Error::new_inner)?;

        let acquire_image_semaphore = device.create_semaphore().map_err(Error::new_inner)?;
        let copy_complete_semaphore = device.create_semaphore().map_err(Error::new_inner)?;
        let render_complete_semaphore = device.create_semaphore().map_err(Error::new_inner)?;

        #[cfg_attr(not(debug_assertions), allow(unused_mut))]
        let mut draw_fence = device
            .create_fence(VulkanFenceCreateFlag::Signalled)
            .map_err(Error::new_inner)?;
        #[cfg(debug_assertions)]
        device
            .set_object_name(
                &mut draw_fence,
                &CString::new(format!("Draw Fence {}", index)).unwrap(),
            )
            .map_err(Error::new_inner)?;

        let transient_buffer = FrameGraphTransientBuffer::new(
            index,
            device.fixed_render_objects(),
            device,
            device.memory_properties(),
        )?;

        Ok(FrameData {
            copy_command_buffer,
            render_command_buffer,

            acquire_image_semaphore,
            copy_complete_semaphore,
            render_complete_semaphore,
            draw_fence,

            transient_buffer,
        })
    }
}
