use crate::{
    Result,
    graphics::{
        Camera, CameraProjection, GraphicsContext, Material, Mesh, Shader, ShaderSource, Vertex,
    },
    math::Color3f,
};

impl GraphicsContext {
    /// Create a new [`Camera`]
    pub fn create_camera(&mut self, projection: CameraProjection) -> Result<Camera> {
        self.managed_objects
            .create_camera(projection, self.size, &self.device)
    }

    /// Create a new [`Shader`]
    pub fn create_shader(&mut self, source: &ShaderSource) -> Result<Shader> {
        self.managed_objects.create_shader(source, &self.device)
    }

    /// Create a new opaque [`Material`]
    pub fn create_opaque_material(&mut self, shader: Shader, color: Color3f) -> Result<Material> {
        self.managed_objects
            .create_opaque_material(shader, color, &self.device)
    }

    /// Create a new [`Mesh`]
    pub fn create_mesh(&mut self, vertices: &[Vertex], indices: &[u32]) -> Result<Mesh> {
        self.managed_objects
            .create_mesh(vertices, indices, &self.device)
    }

    /// Create a new [`Mesh`] without checking the values
    pub unsafe fn create_mesh_unchecked(
        &mut self,
        vertices: &[Vertex],
        indices: &[u32],
    ) -> Result<Mesh> {
        unsafe {
            self.managed_objects
                .create_mesh_unchecked(vertices, indices, &self.device)
        }
    }
}
