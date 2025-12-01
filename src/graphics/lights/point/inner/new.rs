use crate::{
    graphics::lights::PointLightInner,
    math::{Color3f, Vector3f},
};

impl PointLightInner {
    /// Create a new [`PointLightInner`]
    pub(in crate::graphics::lights::point) fn new(
        position: Vector3f,
        radius: f32,
        color: Color3f,
        brightness: f32,
    ) -> Self {
        PointLightInner {
            position,
            radius,
            color,
            brightness,
            dirty: true,
        }
    }
}
