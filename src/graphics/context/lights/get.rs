use crate::{
    graphics::{
        context::Lights,
        lights::{
            DirectionalLight, DirectionalLightHandle, PointLight, PointLightHandle, SpotLight,
            SpotLightHandle,
        },
    },
    math::Color3f,
};

impl Lights {
    /// Get the ambient light color
    pub fn ambient_color(&self) -> Color3f {
        self.constant_buffer.ambient_color
    }

    /// Get the intensity of the ambient light
    pub fn ambient_intensity(&self) -> f32 {
        self.constant_buffer.ambient_intensity
    }

    /// Get the [`DirectionalLight`] at `handle`
    pub fn get_directional_light(
        &self,
        handle: DirectionalLightHandle,
    ) -> Option<&DirectionalLight> {
        self.directional_lights.get(handle)
    }

    /// Get the [`DirectionalLight`] at `handle` mutably
    pub fn get_directional_light_mut(
        &mut self,
        handle: DirectionalLightHandle,
    ) -> Option<&mut DirectionalLight> {
        self.directional_lights.get_mut(handle)
    }

    /// Get the [`DirectionalLight`] at `handle`
    pub fn directional_light(&self, handle: DirectionalLightHandle) -> &DirectionalLight {
        &self.directional_lights[handle]
    }

    /// Get the [`DirectionalLight`] at `handle` mutably
    pub fn directional_light_mut(
        &mut self,
        handle: DirectionalLightHandle,
    ) -> &mut DirectionalLight {
        &mut self.directional_lights[handle]
    }

    /// Get the [`PointLight`] at `handle`
    pub fn get_point_light(&self, handle: PointLightHandle) -> Option<&PointLight> {
        self.point_lights.get(handle)
    }

    /// Get the [`PointLight`] at `handle` mutably
    pub fn get_point_light_mut(&mut self, handle: PointLightHandle) -> Option<&mut PointLight> {
        self.point_lights.get_mut(handle)
    }

    /// Get the [`PointLight`] at `handle`
    pub fn point_light(&self, handle: PointLightHandle) -> &PointLight {
        &self.point_lights[handle]
    }

    /// Get the [`PointLight`] at `handle` mutably
    pub fn point_light_mut(&mut self, handle: PointLightHandle) -> &mut PointLight {
        &mut self.point_lights[handle]
    }

    /// Get the [`SpotLight`] at `handle`
    pub fn get_spot_light(&self, handle: SpotLightHandle) -> Option<&SpotLight> {
        self.spot_lights.get(handle)
    }

    /// Get the [`SpotLight`] at `handle` mutably
    pub fn get_spot_light_mut(&mut self, handle: SpotLightHandle) -> Option<&mut SpotLight> {
        self.spot_lights.get_mut(handle)
    }

    /// Get the [`SpotLight`] at `handle`
    pub fn spot_light(&self, handle: SpotLightHandle) -> &SpotLight {
        &self.spot_lights[handle]
    }

    /// Get the [`SpotLight`] at `handle` mutably
    pub fn spot_light_mut(&mut self, handle: SpotLightHandle) -> &mut SpotLight {
        &mut self.spot_lights[handle]
    }
}
