use crate::{
    Result,
    graphics::{
        Camera, CameraProjection, GraphicsContext, Material, Mesh, MeshRenderer, Shader,
        ShaderSource,
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
    pub fn create_opaque_material(
        &mut self,
        shader: Shader,
        color: Color3f,
        specular_strength: f32,
    ) -> Result<Material> {
        self.managed_objects
            .create_opaque_material(shader, color, specular_strength, &self.device)
    }

    /// Create a new [`MeshRenderer`]
    pub fn create_mesh_renderer(
        &mut self,
        material: Material,
        mesh: Mesh,
        max_instances: usize,
    ) -> Result<MeshRenderer> {
        self.managed_objects
            .create_mesh_renderer(material, mesh, max_instances, &self.device)
    }
}
