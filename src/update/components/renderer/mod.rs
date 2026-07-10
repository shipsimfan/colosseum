use crate::render::MaterialId;

mod get;
mod new;
mod set;
mod system;

/// A component for rendering an object
pub struct Renderer {
    /// The material used to render the object
    material: MaterialId,
}
