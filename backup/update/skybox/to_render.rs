use crate::{
    render::{Mesh, RenderSkybox},
    update::{ECS, Skybox},
};
use alexandria::Id;

impl Skybox {
    /// Convert the skybox to a renderable format
    pub(in crate::update) fn to_render(&self, ecs: &ECS, cube_mesh: Id<Mesh>) -> RenderSkybox {
        match self {
            &Skybox::SolidColor(color) => RenderSkybox::SolidColor(color),
            Skybox::Procedural(procedural) => procedural.to_render(ecs, cube_mesh),
        }
    }
}
