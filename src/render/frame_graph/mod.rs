use nodes::*;

mod nodes;

mod build;
mod compile;
mod execute;
mod new;
mod reset;

/// A frame graph, which can be built and executed to render a frame
pub(in crate::render) struct FrameGraph {
    /// The nodes that have been added to the frame graph
    nodes: Vec<FrameGraphNode>,
}
