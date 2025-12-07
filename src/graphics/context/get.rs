use crate::{
    graphics::{
        Camera, CameraHandle, GraphicsContext, Material, MaterialHandle, MeshRenderer,
        MeshRendererHandle, Shader,
        lights::{
            DirectionalLight, DirectionalLightHandle, PointLight, PointLightHandle, SpotLight,
            SpotLightHandle,
        },
    },
    math::Color3f,
};

impl GraphicsContext {
    /// Get the [`Camera`] at `handle`
    pub fn get_camera(&self, handle: CameraHandle) -> Option<&Camera> {
        self.cameras.get(handle)
    }

    /// Get the [`Camera`] at `handle` mutably
    pub fn get_camera_mut(&mut self, handle: CameraHandle) -> Option<&mut Camera> {
        self.cameras.get_mut(handle)
    }

    /// Get the [`Camera`] at `handle`
    pub fn camera(&self, handle: CameraHandle) -> &Camera {
        &self.cameras[handle]
    }

    /// Get the [`Camera`] at `handle` mutably
    pub fn camera_mut(&mut self, handle: CameraHandle) -> &mut Camera {
        &mut self.cameras[handle]
    }

    /// Get the [`Material`] at `handle`
    pub fn get_opaque_material(&self, handle: MaterialHandle) -> Option<&Material> {
        self.opaque_materials.get(handle)
    }

    /// Get the [`Material`] at `handle` mutably
    pub fn get_opaque_material_mut(&mut self, handle: MaterialHandle) -> Option<&mut Material> {
        self.opaque_materials.get_mut(handle)
    }

    /// Get the [`Material`] at `handle`
    pub fn opaque_material(&self, handle: MaterialHandle) -> &Material {
        &self.opaque_materials[handle]
    }

    /// Get the [`Material`] at `handle` mutably
    pub fn opaque_material_mut(&mut self, handle: MaterialHandle) -> &mut Material {
        &mut self.opaque_materials[handle]
    }

    /// Get the [`MeshRenderer`] at `handle`
    pub fn get_mesh_renderer(&self, handle: MeshRendererHandle) -> Option<&MeshRenderer> {
        self.mesh_renderers.get(handle)
    }

    /// Get the [`MeshRenderer`] at `handle` mutably
    pub fn get_mesh_renderer_mut(
        &mut self,
        handle: MeshRendererHandle,
    ) -> Option<&mut MeshRenderer> {
        self.mesh_renderers.get_mut(handle)
    }

    /// Get the [`MeshRenderer`] at `handle`
    pub fn mesh_renderer(&self, handle: MeshRendererHandle) -> &MeshRenderer {
        &self.mesh_renderers[handle]
    }

    /// Get the [`MeshRenderer`] at `handle` mutably
    pub fn mesh_renderer_mut(&mut self, handle: MeshRendererHandle) -> &mut MeshRenderer {
        &mut self.mesh_renderers[handle]
    }

    /// Get the [`DirectionalLight`] at `handle`
    pub fn get_directional_light(
        &self,
        handle: DirectionalLightHandle,
    ) -> Option<&DirectionalLight> {
        self.lights.get_directional_light(handle)
    }

    /// Get the [`DirectionalLight`] at `handle` mutably
    pub fn get_directional_light_mut(
        &mut self,
        handle: DirectionalLightHandle,
    ) -> Option<&mut DirectionalLight> {
        self.lights.get_directional_light_mut(handle)
    }

    /// Get the [`DirectionalLight`] at `handle`
    pub fn directional_light(&self, handle: DirectionalLightHandle) -> &DirectionalLight {
        &self.lights.directional_light(handle)
    }

    /// Get the [`DirectionalLight`] at `handle` mutably
    pub fn directional_light_mut(
        &mut self,
        handle: DirectionalLightHandle,
    ) -> &mut DirectionalLight {
        self.lights.directional_light_mut(handle)
    }

    /// Get the [`PointLight`] at `handle`
    pub fn get_point_light(&self, handle: PointLightHandle) -> Option<&PointLight> {
        self.lights.get_point_light(handle)
    }

    /// Get the [`PointLight`] at `handle` mutably
    pub fn get_point_light_mut(&mut self, handle: PointLightHandle) -> Option<&mut PointLight> {
        self.lights.get_point_light_mut(handle)
    }

    /// Get the [`PointLight`] at `handle`
    pub fn point_light(&self, handle: PointLightHandle) -> &PointLight {
        &self.lights.point_light(handle)
    }

    /// Get the [`PointLight`] at `handle` mutably
    pub fn point_light_mut(&mut self, handle: PointLightHandle) -> &mut PointLight {
        self.lights.point_light_mut(handle)
    }

    /// Get the [`SpotLight`] at `handle`
    pub fn get_spot_light(&self, handle: SpotLightHandle) -> Option<&SpotLight> {
        self.lights.get_spot_light(handle)
    }

    /// Get the [`SpotLight`] at `handle` mutably
    pub fn get_spot_light_mut(&mut self, handle: SpotLightHandle) -> Option<&mut SpotLight> {
        self.lights.get_spot_light_mut(handle)
    }

    /// Get the [`SpotLight`] at `handle`
    pub fn spot_light(&self, handle: SpotLightHandle) -> &SpotLight {
        &self.lights.spot_light(handle)
    }

    /// Get the [`SpotLight`] at `handle` mutably
    pub fn spot_light_mut(&mut self, handle: SpotLightHandle) -> &mut SpotLight {
        self.lights.spot_light_mut(handle)
    }

    /// Get the default lit shader
    pub fn default_lit_shader(&self) -> Shader {
        self.default_lit_shader.clone()
    }

    /// Get the default unlit shader
    pub fn default_unlit_shader(&self) -> Shader {
        self.default_unlit_shader.clone()
    }

    /// Get the default lit material
    pub fn default_lit_material(&self) -> MaterialHandle {
        self.default_lit_material
    }

    /// Get the default unlit material
    pub fn default_unlit_material(&self) -> MaterialHandle {
        self.default_unlit_material
    }

    /// Get the ambient light color
    pub fn ambient_color(&self) -> Color3f {
        self.lights.ambient_color()
    }

    /// Get the intensity of the ambient light
    pub fn ambient_intensity(&self) -> f32 {
        self.lights.ambient_intensity()
    }
}
