use crate::render::frame_graph::{FrameGraphExternalResourceInfo, FrameGraphTransientResourceInfo};

mod create;
mod finish;
mod get;
mod new;

/// The collection of resources used by the frame graph
pub(in crate::render::frame_graph::builder) struct FrameGraphBuilderResources {
    /// The information about external resources used within the frame graph
    external_resources: Vec<FrameGraphExternalResourceInfo>,

    /// The transient native scale resources that need to be allocated
    native_scale_resources: Vec<FrameGraphTransientResourceInfo>,

    /// The transient render scale resources that need to be allocated
    render_scale_resources: Vec<FrameGraphTransientResourceInfo>,
}
