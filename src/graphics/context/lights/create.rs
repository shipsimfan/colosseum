use crate::{
    graphics::{
        context::Lights,
        lights::{
            DirectionalLight, DirectionalLightHandle, PointLight, PointLightHandle, SpotLight,
            SpotLightHandle,
        },
    },
    math::{Color3f, Vector3f},
};

impl Lights {
    /// Create a new [`DirectionalLight`]
    pub fn create_directional_light(
        &mut self,
        direction: Vector3f,
        color: Color3f,
        brightness: f32,
    ) -> DirectionalLightHandle {
        self.directional_lights
            .insert(DirectionalLight::new(direction, color, brightness))
    }

    /// Create a new [`PointLight`]
    pub fn create_point_light(
        &mut self,
        position: Vector3f,
        radius: f32,
        color: Color3f,
        brightness: f32,
    ) -> PointLightHandle {
        self.point_lights
            .insert(PointLight::new(position, radius, color, brightness))
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
        self.spot_lights.insert(SpotLight::new(
            position,
            distance,
            direction,
            inner_angle,
            outer_angle,
            color,
            brightness,
        ))
    }
}
