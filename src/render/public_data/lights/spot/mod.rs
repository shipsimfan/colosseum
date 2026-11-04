use alexandria::math::{Color3f, Linear, Vector3f};

mod gpu;
mod gpu_matrix;

mod create_buffer;
mod new;
mod set;
mod system;

pub(in crate::render) use gpu::*;
pub(in crate::render) use gpu_matrix::*;

/// A component representing a spot light
pub struct SpotLight {
    /// The color of the spot light
    pub color: Color3f<Linear>,

    /// The intensity of the light
    pub intensity: f32,

    /// The position of the light
    pub position: Vector3f,

    /// The range of the light
    pub range: f32,

    /// The direction of the light
    pub direction: Vector3f,

    /// The cut-off angle of the light
    pub cutoff_angle: f32,

    /// The angle at which lights starts falling off
    pub falloff_angle: f32,
}
