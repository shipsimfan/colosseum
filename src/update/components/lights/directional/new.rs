use crate::update::components::DirectionalLight;
use alexandria::math::{Color3f, Srgb, Vector3f};

impl DirectionalLight {
    /// Create a new [`DirectionalLight`] component
    pub fn new<C: Into<Color3f<Srgb>>, D: Into<Vector3f>>(
        color: C,
        intensity: f32,
        direction: D,
    ) -> DirectionalLight {
        DirectionalLight {
            color: color.into().into_linear(),
            intensity,
            direction: direction.into().normalized(),
        }
    }
}
