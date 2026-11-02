use crate::render::{MaterialId, Mesh};
use alexandria::Id;

mod gpu;

mod create_buffer;
mod get;
mod new;
mod set;
mod system;

pub(in crate::render) use gpu::GpuRenderable;

/// A component for rendering an object
pub struct Renderer {
    /// The material used to render the object
    material: MaterialId,

    /// The mesh used to render the object
    mesh: Id<Mesh>,
}
