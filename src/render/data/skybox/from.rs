use crate::render::Skybox;
use alexandria::math::{Color3f, Linear};

impl<T: Into<Color3f<Linear>>> From<T> for Skybox {
    fn from(color: T) -> Self {
        Skybox::SolidColor(color.into())
    }
}
