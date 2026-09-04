use crate::update::{Entity, ProceduralSkybox};
use alexandria::{
    Id,
    math::{Color3f, Srgb, Vector3f},
};

impl ProceduralSkybox {
    /// Set the color of the sky
    pub fn set_sky_color<C: Into<Color3f<Srgb>>>(&mut self, sky_color: C) {
        self.sky_color = sky_color.into().into_linear();
    }

    /// Set the directional light to follow
    pub fn set_directional_light(&mut self, directional_light: Option<Id<Entity>>) {
        self.directional_light = directional_light;
    }

    /// Set the direction of the sun
    ///
    /// This is ignored if there is a directional light to follow
    pub fn set_sun_direction<D: Into<Vector3f>>(&mut self, sun_direction: D) {
        self.sun_direction = sun_direction.into().normalized();
    }

    /// Set the size of the sun
    pub fn set_sun_size(&mut self, sun_size: f32) {
        self.sun_size = sun_size;
    }

    /// Set the sharpness of the sun's appearance
    pub fn set_sun_sharpness(&mut self, sun_sharpness: f32) {
        self.sun_sharpness = sun_sharpness;
    }

    /// Set the color of the sun
    ///
    /// This is ignored if there is a directional light to follow
    pub fn set_sun_color<C: Into<Color3f<Srgb>>>(&mut self, sun_color: C) {
        self.sun_color = sun_color.into().into_linear();
    }

    /// Set the thickness of the atmosphere
    pub fn set_atmosphere_thickness(&mut self, atmosphere_thickness: f32) {
        self.atmosphere_thickness = atmosphere_thickness;
    }

    /// Set the color of the ground
    pub fn set_ground_color<C: Into<Color3f<Srgb>>>(&mut self, ground_color: C) {
        self.ground_color = ground_color.into().into_linear();
    }
}
