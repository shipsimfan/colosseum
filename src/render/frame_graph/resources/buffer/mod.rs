use crate::render::frame_graph::FrameGraphTransientResource;
use alexandria::gpu::VulkanDeviceMemory;

mod new;

/// A buffer for transient resources used by the frame graph in a specific frame
pub(in crate::render) struct FrameGraphTransientBuffer {
    /// The epoch this buffer is setup for
    pub(in crate::render::frame_graph::resources) epoch: u64,

    /// The transient resources that are at the render scale
    pub(in crate::render::frame_graph::resources) transient_render_scale:
        Vec<FrameGraphTransientResource>,

    /// The memory for the transient render scale resources
    pub(in crate::render::frame_graph::resources) transient_render_scale_memory:
        Option<VulkanDeviceMemory>,

    /// The transient resources that are at the native scale
    pub(in crate::render::frame_graph::resources) transient_native_scale:
        Vec<FrameGraphTransientResource>,

    /// The memory for the transient native scale resources
    pub(in crate::render::frame_graph::resources) transient_native_scale_memory:
        Option<VulkanDeviceMemory>,
}
