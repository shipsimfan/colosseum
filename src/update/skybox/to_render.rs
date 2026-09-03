use crate::{
    render::{Mesh, RenderSkybox},
    update::Skybox,
};
use alexandria::Id;

impl Skybox {
    /// Convert the skybox to a renderable format
    pub(in crate::update) fn to_render(&self, cube_mesh: Id<Mesh>) -> RenderSkybox {
        match self {
            Skybox::SolidColor(color) => RenderSkybox::SolidColor(*color),
            Skybox::Procedural => RenderSkybox::Procedural { mesh: cube_mesh },
        }
    }
}
