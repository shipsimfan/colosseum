use crate::render::{Skybox, frame_graph::FrameGraphSkybox};

impl FrameGraphSkybox {
    /// Convert a [`Skybox`] into a [`FrameGraphSkybox`]
    pub fn from_data(skybox: &Skybox) -> FrameGraphSkybox {
        match skybox {
            Skybox::SolidColor(_) => FrameGraphSkybox::SolidColor,
        }
    }
}
