use crate::render::{RenderSkybox, frame_graph::FrameGraphSkybox};

impl FrameGraphSkybox {
    /// Convert a [`RenderSkybox`] into a [`FrameGraphSkybox`]
    pub fn from_data(skybox: &RenderSkybox) -> FrameGraphSkybox {
        match skybox {
            RenderSkybox::SolidColor(_) => FrameGraphSkybox::SolidColor,
            RenderSkybox::Procedural { .. } => FrameGraphSkybox::Procedural,
        }
    }
}
