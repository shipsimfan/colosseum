use win32::{ComPtr, d3d11::ID3D11Buffer};

mod bind;
mod get;
mod new;

/// A 3d model
pub struct MeshInner {
    /// The buffer for the vertices of the mesh
    vertex_buffer: ComPtr<ID3D11Buffer>,

    /// The buffer for the indices of the mesh
    index_buffer: ComPtr<ID3D11Buffer>,

    /// The number of indices in the buffer
    index_count: u32,
}
