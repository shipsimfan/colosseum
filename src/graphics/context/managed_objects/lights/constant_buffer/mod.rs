use content::LightCbContent;
use win32::{ComPtr, d3d11::ID3D11Buffer};

mod content;

mod bind;
mod get;
mod new;
mod set;

/// The lighting constant buffer
pub(in crate::graphics::context::managed_objects::lights) struct LightConstantBuffer {
    /// The content of the constant buffer
    content: LightCbContent,

    /// The constant buffer on the GPU
    buffer: ComPtr<ID3D11Buffer>,

    /// Has the content of the constant buffer changed?
    dirty: bool,
}
