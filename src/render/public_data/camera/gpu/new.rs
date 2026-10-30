use crate::render::{CameraProjection, GpuCamera};
use alexandria::math::{Matrix4x4f, Quaternionf, Vector3f};

impl GpuCamera {
    /// Create a new GPU camera
    pub(in crate::render::public_data::camera) fn new(
        position: Vector3f,
        rotation: Quaternionf,
        projection: &CameraProjection,
        aspect: f32,
    ) -> GpuCamera {
        let view = Matrix4x4f::from_rotation(rotation.conjugate())
            * Matrix4x4f::from_translation(-position);
        let projection = projection.matrix(aspect);

        let view_projection = projection * view;

        GpuCamera {
            view_projection,
            position,
        }
    }
}
