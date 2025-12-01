use crate::{
    graphics::lights::SpotLightInner,
    math::{Color3f, Vector3f},
};

impl SpotLightInner {
    /// Get the position of the light
    pub fn position(&self) -> Vector3f {
        self.position
    }

    /// Get the distance the light shines
    pub fn distance(&self) -> f32 {
        self.distance
    }

    /// Get the direction the light faces
    pub fn direction(&self) -> Vector3f {
        self.direction
    }

    /// Get the cut-off angle of the light
    pub fn cut_off(&self) -> f32 {
        self.cut_off
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
