use crate::{
    graphics::lights::DirectionalLightInner,
    math::{Color3f, Vector3f},
};

impl DirectionalLightInner {
    /// Get the direction the light faces
    pub fn direction(&self) -> Vector3f {
        self.direction
    }

    /// Get the color of the directional light
    pub fn color(&self) -> Color3f {
        self.color
    }
}
