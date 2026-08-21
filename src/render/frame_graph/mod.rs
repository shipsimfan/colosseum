use alexandria::{
    gpu::{VulkanDeviceMemory, VulkanImageMemoryBarrier, VulkanRenderingAttachmentInfo},
    math::Vector2u,
};
use arena::*;
use pipeline_barrier::*;
use resources::*;
use structure::*;

mod arena;
mod nodes;
mod pipeline_barrier;
mod resources;
mod structure;

mod build;
mod build_and_run;
mod compile;
mod execute;
mod new;

pub(in crate::render) use nodes::*;

/// A frame graph, which can be built and executed to render a frame
pub(in crate::render) struct FrameGraph {
    /// The structure of the frame graph used previously
    structure: Option<FrameGraphStructure>,

    /// The last swapchain size used to build the frame graph
    last_swapchain_size: Vector2u,

    /// The final state of the swapchain image after executing the frame graph
    swapchain_final_state: FrameGraphResourceState,

    /** Build Members **/

    /// The nodes that have been added to the frame graph
    nodes: Vec<FrameGraphNode>,

    /// The external resources that have been added to the frame graph
    external_resources: ArenaBuffer<FrameGraphExternalResource<'static>>,

    /// The info describing the transient resources that are at the render scale
    transient_render_scale_info: Vec<FrameGraphDynamicTransientResourceInfo>,

    /// The transient resources that are at the render scale
    transient_render_scale: Vec<FrameGraphTransientResource>,

    /// The memory for the transient render scale resources
    transient_render_scale_memory: Option<VulkanDeviceMemory>,

    /** Compile Members **/

    /// The node index for needed pipeline barriers and the number of pipeline barriers needed for each node
    pipeline_barrier_indices: Vec<(usize, usize)>,

    /// The actual pipeline barriers needed for each node, in the order they need to be executed
    pipeline_barriers: Vec<FrameGraphPipelineBarrier>,

    /** Execute Members **/

    /// A buffer for image pipeline barriers during the frame graph execution
    image_barriers: ArenaBuffer<VulkanImageMemoryBarrier<'static>>,

    /// A buffer for color attachments during the frame graph execution
    color_attachments: ArenaBuffer<VulkanRenderingAttachmentInfo<'static>>,
}
