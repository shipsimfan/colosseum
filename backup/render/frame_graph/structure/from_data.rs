use crate::render::{
    RenderData,
    frame_graph::{FrameGraphStructure, structure::FrameGraphSkybox},
};

impl FrameGraphStructure {
    /// Convert a [`RenderData`] into a [`FrameGraphStructure`]
    pub fn from_data(data: &RenderData) -> FrameGraphStructure {
        FrameGraphStructure {
            has_render_scale: data.render_scale() != 1.0,
            skybox: FrameGraphSkybox::from_data(data.skybox()),
            anti_aliasing: data.anti_aliasing(),
        }
    }
}
