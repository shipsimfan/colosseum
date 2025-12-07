use crate::{
    graphics::lights::SpotLight,
    math::{Color3f, Vector3f},
};

impl SpotLight {
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

    /// Set the angle to which the light shines with full intensity
    pub fn set_inner_angle(&mut self, inner_angle: f32) {
        self.inner_angle = inner_angle;
    }

    /// Set the angle at which the light stops illuminating completely
    pub fn set_outer_angle(&mut self, outer_angle: f32) {
        self.outer_angle = outer_angle;
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
