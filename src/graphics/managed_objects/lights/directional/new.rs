use crate::{
    graphics::DirectionalLight,
    math::{Color3f, Vector3f},
};

impl DirectionalLight {
    /// Create a new [`DirectionalLight`]
    pub(in crate::graphics::managed_objects::lights::directional) fn new(
        direction: Vector3f,
        color: Color3f,
        brightness: f32,
    ) -> Self {
        DirectionalLight {
            direction: direction.normalized(),
            color,
            brightness,
            dirty: true,
        }
    }
}
