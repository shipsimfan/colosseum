use std::marker::PhantomData;
use win32::{ComPtr, d3d11::ID3D11Buffer};

/// A buffer which contains a set of vertices
pub struct VertexBuffer<T: Sized> {
    /// The buffer itself
    buffer: ComPtr<ID3D11Buffer>,

    /// A placeholder for the vertex type
    _type: PhantomData<T>,
}
