use crate::{
    graphics::PointLight,
    math::{Color3f, Vector3f},
};

impl PointLight {
    /// Create a new [`PointLight`]
    pub(in crate::graphics::managed_objects::lights::point) fn new(
        position: Vector3f,
        radius: f32,
        color: Color3f,
        brightness: f32,
    ) -> Self {
        PointLight {
            position,
            radius,
            color,
            brightness,
            dirty: true,
        }
    }
}
