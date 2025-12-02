use crate::{
    graphics::lights::SpotLightInner,
    math::{Color3f, Vector3f},
};

impl SpotLightInner {
    /// Create a new [`SpotLightInner`]
    pub(in crate::graphics::lights::spot) fn new(
        position: Vector3f,
        distance: f32,
        direction: Vector3f,
        inner_angle: f32,
        outer_angle: f32,
        color: Color3f,
        brightness: f32,
    ) -> Self {
        SpotLightInner {
            position,
            distance,
            direction: direction.normalized(),
            inner_angle,
            outer_angle,
            color,
            brightness,
            dirty: true,
        }
    }
}
