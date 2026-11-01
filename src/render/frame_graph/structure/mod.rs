use crate::render::AntiAliasing;

mod skybox_type;

mod get;
mod new;

pub(in crate::render::frame_graph) use skybox_type::*;

/// A description of the structure of a frame graph
#[derive(PartialEq, Eq)]
pub(in crate::render::frame_graph) struct FrameGraphStructure {
    /// Is render scaling required?
    render_scale: bool,

    /// The type of anti-aliasing being used
    anti_aliasing: AntiAliasing,

    /// The type of skybox being used
    skybox: SkyboxType,
}
