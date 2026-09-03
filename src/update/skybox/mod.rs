use alexandria::math::{Color3f, Linear};

mod default;
mod from;
mod to_render;

/// The type of skybox to use, with properties
pub enum Skybox {
    /// The skybox is a solid color
    SolidColor(Color3f<Linear>),

    /// The skybox is procedurally generated
    Procedural,
}
