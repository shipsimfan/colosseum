use crate::{
    logging::Logger,
    render::{FrameGraph, FrameGraphResourcesPool},
};
use adapter_info::VulkanAdapterInfo;
use alexandria::gpu::{VulkanCommandPool, VulkanDevice, VulkanFormat, VulkanQueue, VulkanSurface};

mod adapter_info;

mod allocate_command_buffer;
mod build_and_run_frame_graph;
mod deref;
mod drop;
mod get;
mod get_adapters;
mod new;

/// The graphics device is responsible for managing the Vulkan device and related resources
pub(in crate::render::job) struct GraphicsDevice<'surface> {
    /// The logger for operations related to the graphics device
    logger: Logger,

    /// The Vulkan device
    device: VulkanDevice,

    /// The queue to submit rendering commands to
    queue: VulkanQueue,

    /// The command pool for the graphics queue
    command_pool: VulkanCommandPool,

    /// The surface to use for the swapchain
    surface: &'surface VulkanSurface,

    /// The format of the swapchain images, which is determined when creating the swapchain
    swapchain_format: VulkanFormat,

    /// The frame graph, which can be built and executed to render frames
    frame_graph: FrameGraph,

    /// The pool of resources the frame graph can use to register resources for rendering
    frame_graph_resources_pool: FrameGraphResourcesPool,
}
