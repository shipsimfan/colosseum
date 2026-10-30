use crate::render::GpuCamera;
use alexandria::math::{Matrix4x4f, Vector3f};

impl Default for GpuCamera {
    fn default() -> Self {
        GpuCamera {
            view_projection: Matrix4x4f::IDENTITY,
            position: Vector3f::ZERO,
        }
    }
}
