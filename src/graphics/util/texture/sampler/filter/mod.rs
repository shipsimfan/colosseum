mod max_anisotropy;
mod to_d3d;

/// The type of filtering to use when sampling a texture
#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub enum TextureFilter {
    /// Use bilinear sampling
    #[default]
    Linear,

    /// Use point sample
    Point,

    /// Use anisotropic sampling
    Anisotropic,
}
