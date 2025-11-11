use crate::{
    Result,
    graphics::{Camera, CameraProjection, GraphicsContext, Material, Shader, ShaderSource},
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
}
