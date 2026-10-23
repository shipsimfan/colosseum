use crate::render::AntiAliasing;

mod get;
mod new;

/// A description of the structure of a frame graph
#[derive(PartialEq, Eq)]
pub(in crate::render::frame_graph) struct FrameGraphStructure {
    /// Is render scaling required?
    render_scale: bool,

    /// The type of anti-aliasing being used
    anti_aliasing: AntiAliasing,
}
