use nodes::*;

mod nodes;
mod resources;

mod add_node;
mod build;
mod build_and_run;
mod compile;
mod execute;
mod new;

pub(in crate::render) use resources::*;

/// A frame graph, which can be built and executed to render a frame
pub(in crate::render) struct FrameGraph {
    /// The nodes that have been added to the frame graph
    nodes: Vec<FrameGraphNode>,
}
