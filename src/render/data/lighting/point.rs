use alexandria::math::{Color4f, Linear, Vector3f};

/// The data describing a point light
#[repr(C, align(16))]
pub(crate) struct RenderPointLight {
    /// The color of the light
    pub color: Color4f<Linear>,

    /// The position of the light
    pub position: Vector3f,

    /// The range of the light
    pub range: f32,
}
