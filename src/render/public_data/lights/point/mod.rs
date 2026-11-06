use alexandria::math::{Color3f, Linear, Vector3f};

mod gpu;
mod gpu_matrix;

mod create_buffer;
mod new;
mod set;
mod shadow_map;
mod system;

pub(in crate::render) use gpu::*;
pub(in crate::render) use gpu_matrix::*;

/// A component representing a point light
pub struct PointLight {
    /// The color of the point light
    pub color: Color3f<Linear>,

    /// The intensity of the light
    pub intensity: f32,

    /// The position of the light
    pub position: Vector3f,

    /// The range of the light
    pub range: f32,
}
