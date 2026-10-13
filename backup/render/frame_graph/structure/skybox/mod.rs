mod create_node;
mod from_data;

/// The kind of skybox being requested to be rendered
#[derive(Debug, PartialEq, Eq)]
pub(in crate::render::frame_graph) enum FrameGraphSkybox {
    /// The skybox is a solid color
    SolidColor,

    /// The skybox is procedurally generated
    Procedural,
}
