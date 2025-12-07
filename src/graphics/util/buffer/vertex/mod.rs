use std::marker::PhantomData;
use win32::{ComPtr, d3d11::ID3D11Buffer};

mod bind;
mod new;

/// A buffer which contains a set of vertices
pub(in crate::graphics) struct VertexBuffer<Vertex: Sized> {
    /// The buffer itself
    buffer: ComPtr<ID3D11Buffer>,

    /// The slot to bind the buffer to
    slot: u32,

    /// A placeholder for the vertex type
    _vertex: PhantomData<Vertex>,
}
