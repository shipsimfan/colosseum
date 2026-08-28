use crate::render::Skybox;
use alexandria::math::{Color3f, Srgb};

impl<T: Into<Color3f<Srgb>>> From<T> for Skybox {
    fn from(color: T) -> Self {
        Skybox::SolidColor(color.into().into_linear())
    }
}
