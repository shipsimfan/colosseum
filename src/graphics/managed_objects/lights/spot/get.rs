use crate::{
    graphics::SpotLight,
    math::{Color3f, Vector3f},
};

impl SpotLight {
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

    /// Get the angle to which the light shines with full intensity
    pub fn inner_angle(&self) -> f32 {
        self.inner_angle
    }

    /// Get the angle at which the light stops illuminating completely
    pub fn outer_angle(&self) -> f32 {
        self.outer_angle
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
