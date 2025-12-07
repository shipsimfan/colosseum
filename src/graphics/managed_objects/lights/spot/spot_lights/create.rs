use crate::{
    graphics::{SpotLight, SpotLightHandle, SpotLights},
    math::{Color3f, Vector3f},
};

impl SpotLights {
    /// Create a new [`SpotLight`]
    pub fn create(
        &mut self,
        position: Vector3f,
        distance: f32,
        direction: Vector3f,
        inner_angle: f32,
        outer_angle: f32,
        color: Color3f,
        brightness: f32,
    ) -> SpotLightHandle {
        self.list.insert(SpotLight::new(
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
