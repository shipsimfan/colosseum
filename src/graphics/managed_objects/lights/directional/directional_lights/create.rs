use crate::{
    graphics::{DirectionalLight, DirectionalLightHandle, DirectionalLights},
    math::{Color3f, Vector3f},
};

impl DirectionalLights {
    /// Create a new [`DirectionalLight`]
    pub fn create(
        &mut self,
        direction: Vector3f,
        color: Color3f,
        brightness: f32,
    ) -> DirectionalLightHandle {
        self.list
            .insert(DirectionalLight::new(direction, color, brightness))
    }
}
