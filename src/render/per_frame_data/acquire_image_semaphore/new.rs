use crate::{
    Result,
    render::{AcquireImageSemaphore, GraphicsDevice},
};
use alexandria::gpu::{VulkanSemaphore, VulkanSemaphoreType};
use colosseum_core::new_error;
#[cfg(debug_assertions)]
use std::ffi::CString;

impl AcquireImageSemaphore {
    /// Create a new set of [`AcquireImageSemaphore`] objects
    pub fn new(device: &GraphicsDevice) -> Result<AcquireImageSemaphore> {
        Ok(AcquireImageSemaphore {
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
    let mut acquire_image_semaphore = device
        .create_semaphore(VulkanSemaphoreType::Binary)
        .map_err(|error| {
            new_error!(
                "unable to create acquire image semaphore {} - {}",
                index,
                error
            )
        })?;

    #[cfg(debug_assertions)]
    device
        .set_object_name(
            &mut acquire_image_semaphore,
            &CString::new(format!("Acquire Image Semaphore {}", index)).unwrap(),
        )
        .map_err(|error| {
            new_error!(
                "unable to set object name for acquire image semaphore {} - {}",
                index,
                error
            )
        })?;

    Ok(acquire_image_semaphore)
}
