use crate::{
    Result,
    graphics::{
        Camera, CameraProjection, Material, Mesh, Shader, ShaderSource, Vertex,
        context::ManagedGraphicsObjects,
    },
    math::{Color3f, Vector2u},
};
use std::num::NonZeroU32;
use win32::d3d11::ID3D11Device;

impl ManagedGraphicsObjects {
    /// Create a new [`Camera`]
    pub fn create_camera(
        &mut self,
        projection: CameraProjection,
        screen_size: Vector2u,
        device: &ID3D11Device,
    ) -> Result<Camera> {
        Camera::new(self.cameras.clone(), projection, screen_size, device)
    }

    /// Cretae a new [`Shader`]
    pub fn create_shader(
        &mut self,
        source: &ShaderSource,
        device: &ID3D11Device,
    ) -> Result<Shader> {
        let shader = Shader::new(self.next_shader_id, source, device)?;
        self.next_shader_id = unsafe { NonZeroU32::new_unchecked(self.next_shader_id.get() + 1) };
        Ok(shader)
    }

    /// Create a new opaque [`Material`]
    pub fn create_opaque_material(
        &mut self,
        shader: Shader,
        color: Color3f,
        device: &ID3D11Device,
    ) -> Result<Material> {
        Material::new(self.opaque_materials.clone(), shader, color, device)
    }

    /// Create a new [`Mesh`]
    pub fn create_mesh(
        &mut self,
        vertices: &[Vertex],
        indices: &[u32],
        device: &ID3D11Device,
    ) -> Result<Mesh> {
        Mesh::new(vertices, indices, device)
    }

    /// Create a new [`Mesh`] without checking the values
    pub unsafe fn create_mesh_unchecked(
        &mut self,
        vertices: &[Vertex],
        indices: &[u32],
        device: &ID3D11Device,
    ) -> Result<Mesh> {
        unsafe { Mesh::new_unchecked(vertices, indices, device) }
    }
}
