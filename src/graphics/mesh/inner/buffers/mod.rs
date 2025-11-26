use win32::{ComPtr, d3d11::ID3D11Buffer};

mod bind;
mod new;

/// The buffers for a mesh on the GPU
pub(in crate::graphics::mesh::inner) struct MeshBuffers {
    /// The buffer for the vertices of the mesh
    vertex_buffer: ComPtr<ID3D11Buffer>,

    /// The buffer for the indices of the mesh
    index_buffer: ComPtr<ID3D11Buffer>,
}
