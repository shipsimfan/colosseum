use crate::update::{Entity, ProceduralSkybox};
use alexandria::{
    Id,
    math::{Color3f, Srgb, Vector3f},
};

impl ProceduralSkybox {
    /// Create a new [`ProceduralSkybox`]
    pub fn new<
        C1: Into<Color3f<Srgb>>,
        D: Into<Vector3f>,
        C2: Into<Color3f<Srgb>>,
        C3: Into<Color3f<Srgb>>,
    >(
        sky_color: C1,
        sun_direction: D,
        sun_size: f32,
        sun_sharpness: f32,
        sun_color: C2,
        atmosphere_thickness: f32,
        ground_color: C3,
    ) -> ProceduralSkybox {
        ProceduralSkybox {
            sky_color: sky_color.into().into_linear(),
            directional_light: None,
            sun_direction: sun_direction.into().normalized(),
            sun_size,
            sun_sharpness,
            sun_color: sun_color.into().into_linear(),
            atmosphere_thickness,
            ground_color: ground_color.into().into_linear(),
        }
    }

    /// Create a new [`ProceduralSkybox`] following a directional light
    pub fn new_light<C1: Into<Color3f<Srgb>>, C2: Into<Color3f<Srgb>>>(
        sky_color: C1,
        directional_light: Id<Entity>,
        sun_size: f32,
        sun_sharpness: f32,
        atmosphere_thickness: f32,
        ground_color: C2,
    ) -> ProceduralSkybox {
        ProceduralSkybox {
            sky_color: sky_color.into().into_linear(),
            directional_light: Some(directional_light),
            sun_direction: -Vector3f::Y,
            sun_size,
            sun_sharpness,
            sun_color: Color3f::WHITE,
            atmosphere_thickness,
            ground_color: ground_color.into().into_linear(),
        }
    }
}
