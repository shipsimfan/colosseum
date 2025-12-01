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
        cut_off: f32,
        color: Color3f,
        brightness: f32,
    ) -> Self {
        SpotLightInner {
            position,
            distance,
            direction: direction.normalized(),
            cut_off,
            color,
            brightness,
            dirty: true,
        }
    }
}
