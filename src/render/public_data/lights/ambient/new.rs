use crate::render::AmbientLight;
use alexandria::math::{Color3f, Linear};

impl AmbientLight {
    /// Create a new [`AmbientLight`] struct
    pub fn new<C: Into<Color3f<Linear>>>(color: C) -> AmbientLight {
        AmbientLight {
            color: color.into(),
        }
    }
}
