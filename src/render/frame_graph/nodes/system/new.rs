use crate::render::frame_graph::{
    FrameGraphNode, FrameGraphNodeSystem, FrameGraphResourceId, nodes::system::FrameGraphRenderPass,
};
use alexandria::gpu::VulkanAttachmentLoadOp;
use std::{borrow::Cow, marker::PhantomData};

impl<Node: FrameGraphNode, System: 'static + crate::ecs::System>
    FrameGraphNodeSystem<Node, System>
{
    /// Create a new [`FrameGraphNodeSystem`]
    pub fn new(
        name: Cow<'static, str>,
        system: System,
        color_attachment: Option<(FrameGraphResourceId, VulkanAttachmentLoadOp)>,
    ) -> FrameGraphNodeSystem<Node, System> {
        FrameGraphNodeSystem {
            name,
            system,

            frame_index: 0,
            secondary_command_buffer: 0,

            render_pass: FrameGraphRenderPass::new(color_attachment),

            _node: PhantomData,
        }
    }
}
