use crate::graphics::{
    Vertex,
    util::{IndexBuffer, VertexBuffer},
};
use std::{borrow::Cow, cell::OnceCell};

mod bind;
mod get;
mod new;

/// A 3d model
pub struct MeshInner {
    /// The vertices that make up this mesh
    vertices: Cow<'static, [Vertex]>,

    /// The indices defining the faces of the mesh
    indices: Cow<'static, [u32]>,

    /// The GPU buffers for this mesh
    buffers: OnceCell<(VertexBuffer<Vertex>, IndexBuffer)>,
}
