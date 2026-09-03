use crate::render::Mesh;
use alexandria::{
    Id,
    math::{Color3f, Linear},
};

mod default;

/// A skybox that can be rendered in the scene
#[derive(Clone)]
pub enum RenderSkybox {
    /// Clear the screen to a solid color before rendering the scene
    SolidColor(Color3f<Linear>),

    /// A procedurally generated skybox
    Procedural {
        /// The ID of the cube mesh used for rendering
        mesh: Id<Mesh>,
    },
}
