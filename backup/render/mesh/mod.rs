mod render;
mod vertex;

mod get;
mod new;

pub use vertex::*;

pub(crate) use render::*;

/// A mesh is a collection of vertices and indices that define a 3D object
pub struct Mesh {
    /// The vertices of the mesh
    vertices: Vec<Vertex>,

    /// The indices of the mesh
    indices: Vec<u32>,
}
