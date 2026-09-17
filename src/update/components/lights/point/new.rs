use crate::update::components::PointLight;
use alexandria::math::{Color3f, Matrix4x4f, Srgb, Vector3f};

impl PointLight {
    /// Create a new [`PointLight`] component
    pub fn new<C: Into<Color3f<Srgb>>, P: Into<Vector3f>>(
        color: C,
        intensity: f32,
        position: P,
        range: f32,
    ) -> PointLight {
        PointLight {
            color: color.into().into_linear(),
            intensity,
            position: position.into(),
            range,
            view_projections: [Matrix4x4f::ZERO; 6],
            dirty: true,
        }
    }
}
