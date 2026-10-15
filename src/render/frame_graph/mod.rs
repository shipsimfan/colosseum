mod nodes;

mod compile;
mod new;

pub(in crate::render) use nodes::*;

/// A frame graph, which can be built and executed to render a frame
pub(in crate::render) struct FrameGraph {}
