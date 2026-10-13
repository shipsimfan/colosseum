use alexandria::math::{Color4f, Linear, Vector3f};

/// The data describing a spot light
#[repr(C, align(16))]
pub(crate) struct RenderSpotLight {
    /// The color of the light
    pub color: Color4f<Linear>,

    /// The position of the light
    pub position: Vector3f,

    /// The range of the light
    pub range: f32,

    /// The direction of the light
    pub direction: Vector3f,

    /// The cutoff angle of the light
    pub cutoff_angle: f32,

    /// The angle at which light begins falling off
    pub falloff_angle: f32,
}
