use crate::render::frame_graph::FrameGraphNode;
use std::{borrow::Cow, marker::PhantomData};

mod new;
mod system;

/// A system wrapping frame graph nodes
pub(in crate::render::frame_graph) struct FrameGraphNodeSystem<
    Node: FrameGraphNode,
    System: crate::ecs::System,
> {
    /// The name of the system
    name: Cow<'static, str>,

    /// The system being wrapped
    system: System,

    /// The index of the frame index resource
    frame_index: usize,

    /// The index of the secondary command buffer used by the node
    secondary_command_buffer: usize,

    /// A marker for the node type
    _node: PhantomData<Node>,
}
