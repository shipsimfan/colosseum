use crate::{
    logging::Logger,
    render::{FrameGraph, RenderObjects},
};
use adapter_info::VulkanAdapterInfo;
use alexandria::{
    Id,
    gpu::{VulkanCommandBuffer, VulkanCommandPool, VulkanDevice, VulkanFormat, VulkanQueue},
};

mod adapter_info;

mod allocate_command_buffer;
mod apply_changes;
mod build_and_run_frame_graph;
mod deref;
mod drop;
mod get;
mod get_adapters;
mod new;
mod present;
mod submit;

/// The graphics device is responsible for managing the Vulkan device and related resources
pub(in crate::render::job) struct GraphicsDevice {
    /// The logger for operations related to the graphics device
    logger: Logger,

    /// The Vulkan device
    device: VulkanDevice,

    /// The queue to submit rendering commands to
    queue: VulkanQueue,

    /// The command pool for the graphics queue
    command_pool: VulkanCommandPool,

    /// The command buffers that have been allocated in the pool
    command_buffers: Vec<Id<VulkanCommandBuffer>>,

    /// The format of the swapchain images, which is determined when creating the swapchain
    swapchain_format: VulkanFormat,

    /// The frame graph, which can be built and executed to render frames
    frame_graph: FrameGraph,

    /// The render objects that have been created by the update job
    render_objects: RenderObjects,
}
