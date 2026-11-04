use alexandria::math::{Color3f, Linear, Vector3f};

mod gpu;
mod gpu_matrix;

mod create_buffer;
mod get;
mod new;
mod set;
mod system;

pub(in crate::render) use gpu::*;
pub(in crate::render) use gpu_matrix::*;

/// A resource representing a directional light
pub struct DirectionalLight {
    /// The color of the directional light
    color: Color3f<Linear>,

    /// The intensity of the light
    intensity: f32,

    /// The direction of the light
    direction: Vector3f,

    /// The depth to render shadows outside of the camera's frustum in the direction of the light
    shadow_depth: f32,

    /// The lambda value for determining the distribution of shadow cascades
    shadow_lambda: f32,
}
