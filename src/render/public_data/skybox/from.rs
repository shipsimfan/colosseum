use crate::render::Skybox;
use alexandria::math::{Color3f, Linear};

impl<T: Into<Color3f<Linear>>> From<T> for Skybox {
    fn from(value: T) -> Self {
        Skybox::SolidColor(value.into())
    }
}
