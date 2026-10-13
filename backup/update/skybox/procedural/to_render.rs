use crate::{
    render::{Mesh, RenderSkybox},
    update::{ECS, ProceduralSkybox, components::DirectionalLight},
};
use alexandria::Id;

impl ProceduralSkybox {
    /// Convert the procedural skybox to a renderable format
    pub fn to_render(&self, ecs: &ECS, mesh: Id<Mesh>) -> RenderSkybox {
        let (sun_direction, sun_color) = self
            .directional_light
            .map(|light| {
                ecs.try_get::<DirectionalLight>(light)
                    .map(|light| (light.direction(), light.color()))
            })
            .flatten()
            .unwrap_or((self.sun_direction, self.sun_color));

        RenderSkybox::Procedural {
            mesh,
            sky_color: self.sky_color,
            sun_direction,
            sun_color,
            sun_size: self.sun_size,
            sun_sharpness: self.sun_sharpness,
            atmosphere_thickness: self.atmosphere_thickness,
            ground_color: self.ground_color,
        }
    }
}
