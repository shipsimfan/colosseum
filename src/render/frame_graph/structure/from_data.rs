use crate::render::{
    RenderData,
    frame_graph::{FrameGraphStructure, structure::FrameGraphSkybox},
};

impl FrameGraphStructure {
    /// Convert a [`RenderData`] into a [`FrameGraphStructure`]
    pub fn from_data(data: &RenderData) -> FrameGraphStructure {
        FrameGraphStructure {
            skybox: FrameGraphSkybox::from_data(data.skybox()),
        }
    }
}
