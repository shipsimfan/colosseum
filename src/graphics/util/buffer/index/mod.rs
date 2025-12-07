use win32::{ComPtr, d3d11::ID3D11Buffer};

mod bind;
mod new;

/// A buffer which holds indices into a vertex buffer
pub(in crate::graphics) struct IndexBuffer {
    /// The buffer itself
    buffer: ComPtr<ID3D11Buffer>,
}
