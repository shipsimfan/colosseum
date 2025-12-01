use crate::{
    graphics::lights::DirectionalLightInner,
    math::{Color3f, Vector3f},
};

impl DirectionalLightInner {
    /// Set the direction the light faces
    pub fn set_direction(&mut self, direction: Vector3f) {
        self.direction = direction.normalized();
        self.dirty = true;
    }

    /// Set the color of the light
    pub fn set_color(&mut self, color: Color3f) {
        self.color = color;
        self.dirty = true;
    }

    /// Set the brightness of the light
    pub fn set_brightness(&mut self, brightness: f32) {
        self.brightness = brightness;
        self.dirty = true;
    }
}
