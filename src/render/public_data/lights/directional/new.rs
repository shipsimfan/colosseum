use crate::render::DirectionalLight;
use alexandria::math::{Color3f, Srgb, Vector3f};

impl DirectionalLight {
    /// Create a new [`DirectionalLight`] component
    pub fn new<C: Into<Color3f<Srgb>>, D: Into<Vector3f>>(
        color: C,
        intensity: f32,
        direction: D,
        shadow_depth: f32,
        shadow_lambda: f32,
    ) -> DirectionalLight {
        DirectionalLight {
            color: color.into().into_linear(),
            intensity,
            direction: direction.into().normalized(),
            shadow_depth,
            shadow_lambda,
        }
    }
}
