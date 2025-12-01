use crate::{
    graphics::lights::DirectionalLightInner,
    math::{Color3f, Vector3f},
};

impl DirectionalLightInner {
    /// Create a new [`DirectionalLightInner`]
    pub(in crate::graphics::lights::directional) fn new(
        direction: Vector3f,
        brightness: f32,
        color: Color3f,
    ) -> Self {
        DirectionalLightInner {
            direction: direction.normalized(),
            brightness,
            color,
            dirty: true,
        }
    }
}
