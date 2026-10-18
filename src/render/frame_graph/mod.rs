use builder::*;
use nodes::*;
use resources::*;
use submission::*;

mod builder;
mod nodes;
mod resources;
mod submission;

mod compile;
mod new;

pub(in crate::render) use nodes::create_frame_graph_resources;

/// A frame graph, which can be built and executed to render a frame
pub(in crate::render) struct FrameGraph {
    /// Has the frame graph been compiled?
    compiled: bool,
}
