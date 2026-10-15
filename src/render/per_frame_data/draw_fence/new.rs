use crate::{
    Result,
    render::{DrawFence, GraphicsDevice},
};
use alexandria::gpu::VulkanFence;
use colosseum_core::new_error;
#[cfg(debug_assertions)]
use std::ffi::CString;

impl DrawFence {
    /// Create a new set of [`DrawFence`] objects
    pub fn new(device: &GraphicsDevice) -> Result<DrawFence> {
        Ok(DrawFence {
            fences: [
                (create_fence(0, device)?, false),
                (create_fence(1, device)?, false),
                (create_fence(2, device)?, false),
            ],
        })
    }
}

fn create_fence(index: usize, device: &GraphicsDevice) -> Result<VulkanFence> {
    #[cfg_attr(not(debug_assertions), allow(unused_mut))]
    let mut draw_fence = device
        .create_fence(0)
        .map_err(|error| new_error!("unable to create draw fence {} - {}", index, error))?;

    #[cfg(debug_assertions)]
    device
        .set_object_name(
            &mut draw_fence,
            &CString::new(format!("Draw Fence {}", index)).unwrap(),
        )
        .map_err(|error| {
            new_error!(
                "unable to set object name for draw fence {} - {}",
                index,
                error
            )
        })?;

    Ok(draw_fence)
}
