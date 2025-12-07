use crate::{
    Result,
    graphics::{MaterialHandle, Mesh, MeshRenderer, util::InstanceBuffer},
    math::Matrix4x4f,
};
use win32::d3d11::ID3D11Device;

impl MeshRenderer {
    /// Create a new [`MeshRenderer`]
    pub(in crate::graphics) fn new(
        material: MaterialHandle,
        mesh: Mesh,
        max_instances: usize,
        device: &ID3D11Device,
    ) -> Result<Self> {
        let instance_buffer =
            InstanceBuffer::new(Matrix4x4f::identity(), max_instances, 1, device)?;

        Ok(MeshRenderer {
            active: true,
            material,
            mesh,
            instances: Vec::with_capacity(max_instances),
            instance_buffer,
        })
    }
}
