use crate::graphics::TextureFilter;
use win32::d3d11::D3D11_FILTER;

impl TextureFilter {
    /// Get the type of [`D3D11_FILTER`] that should be used
    pub(in crate::graphics::util::texture::sampler) fn to_d3d(&self) -> D3D11_FILTER {
        match self {
            TextureFilter::Linear => D3D11_FILTER::MinMagMipLinear,
            TextureFilter::Point => D3D11_FILTER::MinMagMipPoint,
            TextureFilter::Anisotropic => D3D11_FILTER::Anisotropic,
        }
    }
}
