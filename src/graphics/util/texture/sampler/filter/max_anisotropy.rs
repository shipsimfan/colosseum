use crate::graphics::TextureFilter;
use win32::UINT;

impl TextureFilter {
    /// Get the max amount of anistropy for this filter
    pub(in crate::graphics::util::texture::sampler) fn max_anisotropy(&self) -> UINT {
        match self {
            TextureFilter::Anisotropic => 16,
            _ => 0,
        }
    }
}
