use alexandria::math::{Color3f, Linear};

mod procedural;

mod default;
mod from;

pub use procedural::*;

/// A skybox that can be rendered in the scene
#[derive(Clone)]
pub enum Skybox {
    /// Clear the screen to a solid color before rendering the scene
    SolidColor(Color3f<Linear>),

    /// The skybox is procedurally generated
    Procedural(ProceduralSkybox),
}
