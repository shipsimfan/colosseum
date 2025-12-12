use win32::{ComPtr, d3d11::ID3D11SamplerState};

mod edge;
mod filter;

mod bind;
mod new;

pub use edge::TextureEdge;
pub use filter::TextureFilter;

/// The algorithms to use when sampling a texture
pub(in crate::graphics) struct TextureSampler {
    /// The actual underlying sampler state
    sampler: ComPtr<ID3D11SamplerState>,
}
