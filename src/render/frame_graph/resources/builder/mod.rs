use crate::render::frame_graph::{
    Arena, FrameGraphDynamicTransientResourceInfo, FrameGraphExternalResource,
};

mod clear_transient;
mod create;
mod finish;
mod get;
mod new;
mod set;
mod transition;

/// A builder for creating resources in the frame graph
pub(in crate::render::frame_graph) struct FrameGraphResourceBuilder<'a> {
    /// The external resources that are available to the frame graph, such as the swapchain image
    external: Arena<'a, FrameGraphExternalResource<'a>>,

    /// The transient resources that are at the render scale
    transient_render_scale: &'a mut Vec<FrameGraphDynamicTransientResourceInfo>,
}
