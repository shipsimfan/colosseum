use alexandria::math::{Color4f, Linear, Vector3f};

/// The data describing a directional light
#[repr(C, align(16))]
pub(crate) struct RenderDirectionalLight {
    /// The color of the light
    pub color: Color4f<Linear>,

    /// The direction of the light
    pub direction: Vector3f,

    /// A reserved value for padding
    pub reserved: u32,

    /// The camera-local far depths of the cascades
    pub cascade_far_depths: [f32; 4],
}
