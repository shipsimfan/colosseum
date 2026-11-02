use crate::render::Mesh;
use alexandria::Id;

mod new;
mod push;
mod reset;

/// The renderable item for a specific material type, consisting of the material ID, mesh ID, and
/// an index into the renderable buffer
pub(in crate::render) type Renderable<T> = (Id<T>, Id<Mesh>, usize);

/// A list of renderable items for a specific material type
pub(in crate::render) struct Renderables<T> {
    /// The list of renderable items for this material type
    items: Vec<Renderable<T>>,
}
