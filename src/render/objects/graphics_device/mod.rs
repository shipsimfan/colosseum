use crate::render::RenderGpuTransferQueue;
use adapter_info::*;
use alexandria::gpu::{VulkanAdapterMemoryProperties, VulkanDevice, VulkanFormat, VulkanQueue};
use colosseum_core::Logger;
use std::sync::Arc;

mod adapter_info;

mod allocate_memory;
mod deref;
mod get;
mod get_adapters;
mod new;

/// The graphics device is responsible for managing the Vulkan device and related resources
pub struct GraphicsDevice {
    /// The logger for operations related to the graphics device
    logger: Logger,

    /// The Vulkan device
    device: VulkanDevice,

    /// The queue to submit rendering commands to
    render_queue: VulkanQueue,

    /// The index of the render queue family
    render_queue_family: u32,

    /// The format to use for the swapchain images
    swapchain_format: VulkanFormat,

    /// The memory properties of the device
    memory_properties: Arc<VulkanAdapterMemoryProperties>,

    /// The render GPU transfer queue if there is no dedicated transfer queue
    gpu_transfer_queue: Option<RenderGpuTransferQueue>,
}
