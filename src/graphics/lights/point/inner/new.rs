use crate::{
    graphics::lights::PointLightInner,
    math::{Color3f, Vector3f},
};

impl PointLightInner {
    /// Create a new [`PointLightInner`]
    pub(in crate::graphics::lights::point) fn new(
        position: Vector3f,
        radius: f32,
        brightness: f32,
        color: Color3f,
    ) -> Self {
        PointLightInner {
            position,
            radius,
            brightness,
            color,
            dirty: true,
        }
    }
}
