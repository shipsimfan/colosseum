/// The type of skybox being used
#[derive(Clone, Copy, PartialEq, Eq)]
pub(in crate::render::frame_graph) enum SkyboxType {
    /// The skybox is a solid color
    SolidColor,

    /// The skybox is procedurally generated
    Procedural,
}
