use crate::{logging::Logger, render::data::camera::CameraShaderData};
use alexandria::math::Matrix4x4f;
use std::f32::consts::PI;

impl CameraShaderData {
    /// Create a new [`CameraShaderData`]
    pub fn new(logger: &Logger) -> CameraShaderData {
        CameraShaderData {
            view_projection: Matrix4x4f::new_perspective(16.0 / 9.0, PI / 3.0, 0.001, 1000.0),
        }
    }
}
