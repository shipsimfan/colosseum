use alexandria::math::{Color4f, Linear, Vector3f};

mod new;

/// The data describing a directional light on the GPU
#[repr(C, align(16))]
pub(in crate::render) struct GpuDirectionalLight {
    /// The color of the light
    color: Color4f<Linear>,

    /// The direction of the light
    direction: Vector3f,

    /// A reserved value for padding
    reserved: u32,

    /// The camera-local far depths of the cascades
    cascade_far_depths: [f32; 4],
}
