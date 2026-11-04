use alexandria::math::{Color4f, Linear, Vector3f};

mod new;

/// The data describing a spot light on the GPU
#[repr(C, align(16))]
pub(in crate::render) struct GpuSpotLight {
    /// The color of the light
    color: Color4f<Linear>,

    /// The position of the light
    position: Vector3f,

    /// The range of the light
    range: f32,

    /// The direction of the light
    direction: Vector3f,

    /// The cutoff angle of the light
    cutoff_angle: f32,

    /// The angle at which light begins falling off
    falloff_angle: f32,
}
