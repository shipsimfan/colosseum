use alexandria::gpu::{VulkanImageMemoryBarrier, VulkanRenderingAttachmentInfo};
use resource_usage::*;

mod nodes;
mod resource_usage;
mod resources;

mod add_node;
mod build;
mod build_and_run;
mod compile;
mod execute;
mod new;

pub(in crate::render) use nodes::*;
pub(in crate::render) use resources::*;

/// A frame graph, which can be built and executed to render a frame
pub(in crate::render) struct FrameGraph {
    /// The nodes that have been added to the frame graph
    nodes: Vec<FrameGraphNode>,

    /// A buffer for image pipeline barriers during the frame graph execution
    image_barriers_buffer: Vec<VulkanImageMemoryBarrier<'static>>,

    /// A buffer for color attachments during the frame graph execution
    color_attachments_buffer: Vec<VulkanRenderingAttachmentInfo<'static>>,
}
