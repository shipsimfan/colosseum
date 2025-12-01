use crate::{
    graphics::lights::SpotLightInner,
    math::{Color3f, Vector3f},
};

impl SpotLightInner {
    /// Set the position of the light
    pub fn set_position(&mut self, position: Vector3f) {
        self.position = position;
        self.dirty = true;
    }

    /// Set the distance the light shines
    pub fn set_distance(&mut self, distance: f32) {
        self.distance = distance;
        self.dirty = true;
    }

    /// Set the direction the light faces
    pub fn set_direction(&mut self, direction: Vector3f) {
        self.direction = direction.normalized();
        self.dirty = true;
    }

    /// Set the cut-off angle of the light
    pub fn set_cut_off(&mut self, cut_off: f32) {
        self.cut_off = cut_off;
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
