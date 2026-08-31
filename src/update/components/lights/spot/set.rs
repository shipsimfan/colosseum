use std::f32::consts::PI;

use crate::update::components::SpotLight;
use alexandria::math::{Color3f, Srgb, Vector3f};

impl SpotLight {
    /// Set the color of the spot light
    pub fn set_color<C: Into<Color3f<Srgb>>>(&mut self, color: C) {
        self.color = color.into().into_linear();
    }

    /// Set the intensity of the spot light
    pub fn set_intensity(&mut self, intensity: f32) {
        self.intensity = intensity;
    }

    /// Set the direction of the light
    pub fn set_position<P: Into<Vector3f>>(&mut self, position: P) {
        self.position = position.into();
    }

    /// Set the range of the light
    pub fn set_range(&mut self, range: f32) {
        self.range = range;
    }

    /// Set the direction of the light
    pub fn set_direction<D: Into<Vector3f>>(&mut self, direction: D) {
        self.direction = direction.into().normalized();
    }

    /// Set the cutoff angle of the light
    pub fn set_cutoff_angle(&mut self, cutoff_angle: f32) {
        self.cutoff_angle = cutoff_angle / PI;
    }

    /// Set the angle at which light begins falling off
    pub fn set_falloff_angle(&mut self, falloff_angle: f32) {
        self.falloff_angle = falloff_angle / PI;
    }
}
