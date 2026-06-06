use alexandria::gpu::{VulkanImageMemoryBarrier, VulkanRenderingAttachmentInfo};
use arena::*;
use nodes::*;
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

/// A frame graph, which can be built and executed to render a frame
pub(in crate::render) struct FrameGraph {
    /// The structure of the frame graph used previously
    structure: Option<FrameGraphStructure>,

    /** Build Members **/

    /// The nodes that have been added to the frame graph
    nodes: Vec<FrameGraphNode>,

    /// The external resources that have been added to the frame graph
    external_resources: ArenaBuffer<FrameGraphExternalResource<'static>>,

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
