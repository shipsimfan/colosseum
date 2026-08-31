use std::f32::consts::PI;

use crate::update::components::SpotLight;
use alexandria::math::{Color3f, Srgb, Vector3f};

impl SpotLight {
    /// Create a new [`SpotLight`] component
    pub fn new<C: Into<Color3f<Srgb>>, P: Into<Vector3f>, D: Into<Vector3f>>(
        color: C,
        intensity: f32,
        position: P,
        range: f32,
        direction: D,
        cutoff_angle: f32,
        falloff_angle: f32,
    ) -> SpotLight {
        SpotLight {
            color: color.into().into_linear(),
            intensity,
            position: position.into(),
            range,
            direction: direction.into().normalized(),
            cutoff_angle: cutoff_angle / PI,
            falloff_angle: falloff_angle / PI,
        }
    }
}
