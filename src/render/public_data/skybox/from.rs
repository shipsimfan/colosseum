use crate::render::{ProceduralSkybox, Skybox};
use alexandria::math::{Color3f, Linear};

impl<T: Into<Color3f<Linear>>> From<T> for Skybox {
    fn from(value: T) -> Self {
        Skybox::SolidColor(value.into())
    }
}

impl From<ProceduralSkybox> for Skybox {
    fn from(value: ProceduralSkybox) -> Self {
        Skybox::Procedural(value)
    }
}
