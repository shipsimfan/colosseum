use crate::{
    graphics::lights::PointLightInner,
    math::{Color3f, Vector3f},
};

impl PointLightInner {
    /// Set the position of the light
    pub fn set_position(&mut self, position: Vector3f) {
        self.position = position;
        self.dirty = true;
    }

    /// Set the radius of the light
    pub fn set_radius(&mut self, radius: f32) {
        self.radius = radius;
        self.dirty = true;
    }

    /// Set the brightness of the light
    pub fn set_brightness(&mut self, brightness: f32) {
        self.brightness = brightness;
        self.dirty = true;
    }

    /// Set the color of the light
    pub fn set_color(&mut self, color: Color3f) {
        self.color = color;
        self.dirty = true;
    }
}
