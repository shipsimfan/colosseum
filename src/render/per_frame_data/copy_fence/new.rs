use crate::{
    Result,
    render::{CopyFence, GraphicsDevice},
};
use alexandria::gpu::{VulkanFence, VulkanFenceCreateFlag};
use colosseum_core::new_error;
#[cfg(debug_assertions)]
use std::ffi::CString;

impl CopyFence {
    /// Create a new set of [`CopyFence`] objects
    pub fn new(device: &GraphicsDevice) -> Result<CopyFence> {
        Ok(CopyFence {
            fences: [
                create_fence(0, device)?,
                create_fence(1, device)?,
                create_fence(2, device)?,
            ],
        })
    }
}

fn create_fence(index: usize, device: &GraphicsDevice) -> Result<VulkanFence> {
    #[cfg_attr(not(debug_assertions), allow(unused_mut))]
    let mut copy_fence = device
        .create_fence(VulkanFenceCreateFlag::Signalled)
        .map_err(|error| new_error!("unable to create copy fence {} - {}", index, error))?;

    #[cfg(debug_assertions)]
    device
        .set_object_name(
            &mut copy_fence,
            &CString::new(format!("Copy Fence {}", index)).unwrap(),
        )
        .map_err(|error| {
            new_error!(
                "unable to set object name for copy fence {} - {}",
                index,
                error
            )
        })?;

    Ok(copy_fence)
}
