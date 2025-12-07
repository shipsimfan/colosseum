use crate::{
    Result,
    graphics::{
        Camera, CameraHandle, CameraProjection, GraphicsContext, Material, MaterialHandle, Mesh,
        MeshRenderer, MeshRendererHandle, Shader, ShaderSource,
        lights::{DirectionalLightHandle, PointLightHandle, SpotLightHandle},
    },
    math::{Color3f, Vector3f},
};

impl GraphicsContext {
    /// Create a new [`Camera`]
    pub fn create_camera(&mut self, projection: CameraProjection) -> Result<CameraHandle> {
        Ok(self
            .cameras
            .insert(Camera::new(projection, self.size, &self.device)?))
    }

    /// Create a new unlit [`Shader`]
    pub fn create_unlit_shader(&mut self, source: &ShaderSource) -> Result<Shader> {
        Shader::new_unlit(source, &self.device)
    }

    /// Create a new lit [`Shader`]
    pub fn create_lit_shader(&mut self, source: &ShaderSource) -> Result<Shader> {
        Shader::new_lit(source, &self.device)
    }

    /// Create a new opaque [`Material`]
    pub fn create_opaque_material(
        &mut self,
        shader: Shader,
        color: Color3f,
        specular_strength: f32,
    ) -> Result<MaterialHandle> {
        let id = self.next_material_id;
        self.next_material_id = self.next_material_id.checked_add(1).unwrap();

        Ok(self.opaque_materials.insert(Material::new(
            id,
            shader,
            color,
            specular_strength,
            &self.device,
        )?))
    }

    /// Create a new [`MeshRenderer`]
    pub fn create_mesh_renderer(
        &mut self,
        material: MaterialHandle,
        mesh: Mesh,
        max_instances: usize,
    ) -> Result<MeshRendererHandle> {
        Ok(self.mesh_renderers.insert(MeshRenderer::new(
            material,
            mesh,
            max_instances,
            &self.device,
        )?))
    }

    /// Create a new [`DirectionalLight`]
    pub fn create_directional_light(
        &mut self,
        direction: Vector3f,
        color: Color3f,
        brightness: f32,
    ) -> DirectionalLightHandle {
        self.lights
            .create_directional_light(direction, color, brightness)
    }

    /// Create a new [`PointLight`]
    pub fn create_point_light(
        &mut self,
        position: Vector3f,
        radius: f32,
        color: Color3f,
        brightness: f32,
    ) -> PointLightHandle {
        self.lights
            .create_point_light(position, radius, color, brightness)
    }

    /// Create a new [`SpotLight`]
    pub fn create_spot_light(
        &mut self,
        position: Vector3f,
        distance: f32,
        direction: Vector3f,
        inner_angle: f32,
        outer_angle: f32,
        color: Color3f,
        brightness: f32,
    ) -> SpotLightHandle {
        self.lights.create_spot_light(
            position,
            distance,
            direction,
            inner_angle,
            outer_angle,
            color,
            brightness,
        )
    }
}
