mod id;
mod resource;
mod state;
mod usage;

mod clear;
mod get;
mod insert;
mod new;

pub(in crate::render::frame_graph) use id::*;
pub(in crate::render::frame_graph) use resource::*;
pub(in crate::render::frame_graph) use state::*;
pub(in crate::render::frame_graph) use usage::*;

/// The collection of all resources used within the frame graph
pub(in crate::render::frame_graph) struct FrameGraphResources {
    /// The external resources used within the frame graph
    external_resources: Vec<FrameGraphExternalResource>,
}
