use alexandria::math::{Color3f, Linear};

mod create_node;
mod default;
mod from;

/// A skybox that can be rendered in the scene
pub enum Skybox {
    /// Clear the screen to a solid color before rendering the scene
    SolidColor(Color3f<Linear>),
}
