use alexandria::math::{Color4f, Linear, Vector3f};

mod new;

/// The data describing a point light on the GPU
#[repr(C, align(16))]
pub(in crate::render) struct GpuPointLight {
    /// The color of the light
    color: Color4f<Linear>,

    /// The position of the light
    position: Vector3f,

    /// The range of the light
    range: f32,
}
