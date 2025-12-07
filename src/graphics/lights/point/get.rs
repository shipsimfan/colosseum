use crate::{
    graphics::lights::PointLight,
    math::{Color3f, Vector3f},
};

impl PointLight {
    /// Get the position of the light
    pub fn position(&self) -> Vector3f {
        self.position
    }

    /// Get the radius of the light
    pub fn radius(&self) -> f32 {
        self.radius
    }

    /// Get the color of the point light
    pub fn color(&self) -> Color3f {
        self.color
    }

    /// The brightness of the light
    pub fn brightness(&self) -> f32 {
        self.brightness
    }
}
