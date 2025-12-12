use crate::math::Color3f;

mod border_color;
mod to_d3d;

/// The type of sampling to use at the edge of a texture
#[derive(Default, Clone, Copy, PartialEq)]
pub enum TextureEdge {
    /// Tile the texture at every (u,v) integer junction. For example, for u values between 0 and
    /// 3, the texture is repeated three times.
    #[default]
    Wrap,

    /// Flip the texture at every (u,v) integer junction. For u values between 0 and 1, for
    /// example, the texture is addressed normally; between 1 and 2, the texture is flipped
    /// (mirrored); between 2 and 3, the texture is normal again; and so on.
    Mirror,

    /// Texture coordinates outside the range [0.0, 1.0] are set to the texture color at 0.0 or
    /// 1.0, respectively.
    Clamp,

    /// Similar to [`TextureEdge::Mirror`] and [`TextureEdge::Clamp`]. Takes the absolute value of
    /// the texture coordinate (thus, mirroring around 0), and then clamps to the maximum value.
    MirrorOnce,

    /// Texture coordinates outside the range [0.0, 1.0] are set to the border color specified
    Border(Color3f),
}
