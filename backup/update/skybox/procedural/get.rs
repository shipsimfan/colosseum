use crate::update::{Entity, ProceduralSkybox};
use alexandria::{
    Id,
    math::{Color3f, Linear, Vector3f},
};

impl ProceduralSkybox {
    /// Get the base color of the sky
    pub fn sky_color(&self) -> Color3f<Linear> {
        self.sky_color
    }

    /// Get the directional light to follow
    pub fn directional_light(&self) -> Option<Id<Entity>> {
        self.directional_light
    }

    /// Get the requested direction of the sun
    ///
    /// If there is a directional light to follow, this is ignored
    pub fn sun_direction(&self) -> Vector3f {
        self.sun_direction
    }

    /// Get the size of the sun
    pub fn sun_size(&self) -> f32 {
        self.sun_size
    }

    /// Get the sharpness of the sun's appearance
    pub fn sun_sharpness(&self) -> f32 {
        self.sun_sharpness
    }

    /// Get the color of the sun
    ///
    /// If there is a directional light to follow, this is ignored
    pub fn sun_color(&self) -> Color3f<Linear> {
        self.sun_color
    }

    /// Get the thickness of the atmosphere
    pub fn atmosphere_thickness(&self) -> f32 {
        self.atmosphere_thickness
    }

    /// Get the color of the ground
    pub fn ground_color(&self) -> Color3f<Linear> {
        self.ground_color
    }
}
