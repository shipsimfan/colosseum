use crate::graphics::TextureEdge;
use win32::d3d11::D3D11_TEXTURE_ADDRESS_MODE;

impl TextureEdge {
    /// Get the type of [`D3D11_TEXTURE_ADDRESS_MODE`] that should be used
    pub(in crate::graphics::util::texture::sampler) fn to_d3d(&self) -> D3D11_TEXTURE_ADDRESS_MODE {
        match self {
            TextureEdge::Wrap => D3D11_TEXTURE_ADDRESS_MODE::Wrap,
            TextureEdge::Mirror => D3D11_TEXTURE_ADDRESS_MODE::Mirror,
            TextureEdge::Clamp => D3D11_TEXTURE_ADDRESS_MODE::Clamp,
            TextureEdge::MirrorOnce => D3D11_TEXTURE_ADDRESS_MODE::MirrorOnce,
            TextureEdge::Border(_) => D3D11_TEXTURE_ADDRESS_MODE::Border,
        }
    }
}
