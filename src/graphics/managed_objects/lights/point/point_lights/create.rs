use crate::{
    graphics::{PointLight, PointLightHandle, PointLights},
    math::{Color3f, Vector3f},
};

impl PointLights {
    /// Create a new [`PointLight`]
    pub fn create(
        &mut self,
        position: Vector3f,
        radius: f32,
        color: Color3f,
        brightness: f32,
    ) -> PointLightHandle {
        self.list
            .insert(PointLight::new(position, radius, color, brightness))
    }
}
