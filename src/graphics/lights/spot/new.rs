use crate::{
    graphics::lights::SpotLight,
    math::{Color3f, Vector3f},
};

impl SpotLight {
    /// Create a new [`SpotLight`]
    pub(in crate::graphics) fn new(
        position: Vector3f,
        distance: f32,
        direction: Vector3f,
        inner_angle: f32,
        outer_angle: f32,
        color: Color3f,
        brightness: f32,
    ) -> Self {
        SpotLight {
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
