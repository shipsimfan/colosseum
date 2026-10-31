use crate::{
    Result,
    render::{CopyCompleteSemaphore, GraphicsDevice},
};
use alexandria::gpu::{VulkanSemaphore, VulkanSemaphoreType};
use colosseum_core::new_error;
#[cfg(debug_assertions)]
use std::ffi::CString;

impl CopyCompleteSemaphore {
    /// Create a new set of [`CopyCompleteSemaphore`] objects
    pub fn new(device: &GraphicsDevice) -> Result<CopyCompleteSemaphore> {
        Ok(CopyCompleteSemaphore {
            semaphores: [
                create_semaphore(0, device)?,
                create_semaphore(1, device)?,
                create_semaphore(2, device)?,
            ],
        })
    }
}

fn create_semaphore(index: usize, device: &GraphicsDevice) -> Result<(VulkanSemaphore, u64)> {
    #[cfg_attr(not(debug_assertions), allow(unused_mut))]
    let mut copy_complete_semaphore = device
        .create_semaphore(VulkanSemaphoreType::Timeline(0))
        .map_err(|error| {
            new_error!(
                "unable to create copy complete semaphore {} - {}",
                index,
                error
            )
        })?;

    #[cfg(debug_assertions)]
    device
        .set_object_name(
            &mut copy_complete_semaphore,
            &CString::new(format!("Copy Complete Semaphore {}", index)).unwrap(),
        )
        .map_err(|error| {
            new_error!(
                "unable to set object name for copy complete semaphore {} - {}",
                index,
                error
            )
        })?;

    Ok((copy_complete_semaphore, 0))
}
