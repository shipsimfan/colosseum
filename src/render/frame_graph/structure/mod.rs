mod skybox;

mod from_data;
mod get;

pub(in crate::render::frame_graph) use skybox::*;

/// A representation of the structure of a frame graph so we can quickly tell if we need to recompile the frame graph or not
#[derive(Debug, PartialEq, Eq)]
pub(in crate::render::frame_graph) struct FrameGraphStructure {
    /// The kind of skybox being requested
    skybox: FrameGraphSkybox,
}
