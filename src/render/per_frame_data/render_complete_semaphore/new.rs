use crate::{
    Result,
    render::{GraphicsDevice, RenderCompleteSemaphore},
};
use alexandria::gpu::{VulkanSemaphore, VulkanSemaphoreType};
use colosseum_core::new_error;
#[cfg(debug_assertions)]
use std::ffi::CString;

impl RenderCompleteSemaphore {
    /// Create a new set of [`RenderCompleteSemaphore`] objects
    pub fn new(device: &GraphicsDevice) -> Result<RenderCompleteSemaphore> {
        Ok(RenderCompleteSemaphore {
            semaphores: [
                create_semaphore(0, device)?,
                create_semaphore(1, device)?,
                create_semaphore(2, device)?,
            ],
        })
    }
}

fn create_semaphore(index: usize, device: &GraphicsDevice) -> Result<VulkanSemaphore> {
    #[cfg_attr(not(debug_assertions), allow(unused_mut))]
    let mut render_complete_semaphore = device
        .create_semaphore(VulkanSemaphoreType::Binary)
        .map_err(|error| {
            new_error!(
                "unable to create render complete semaphore {} - {}",
                index,
                error
            )
        })?;

    #[cfg(debug_assertions)]
    device
        .set_object_name(
            &mut render_complete_semaphore,
            &CString::new(format!("Render Complete Semaphore {}", index)).unwrap(),
        )
        .map_err(|error| {
            new_error!(
                "unable to set object name for render complete semaphore {} - {}",
                index,
                error
            )
        })?;

    Ok(render_complete_semaphore)
}
