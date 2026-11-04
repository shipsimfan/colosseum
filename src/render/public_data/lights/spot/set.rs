use crate::render::SpotLight;
use alexandria::math::{Color3f, Srgb, Vector3f};

impl SpotLight {
    /// Set the color of the spot light
    pub fn set_color<C: Into<Color3f<Srgb>>>(&mut self, color: C) {
        self.color = color.into().into_linear();
    }

    /// Set the direction of the light
    pub fn set_position<P: Into<Vector3f>>(&mut self, position: P) {
        self.position = position.into();
    }

    /// Set the direction of the light
    pub fn set_direction<D: Into<Vector3f>>(&mut self, direction: D) {
        self.direction = direction.into().normalized();
    }
}
