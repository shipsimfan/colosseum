use crate::graphics::TextureEdge;

impl TextureEdge {
    /// Get the border colored specified by this edge
    pub(in crate::graphics::util::texture::sampler) fn border_color(&self) -> [f32; 4] {
        match self {
            TextureEdge::Border(border_color) => {
                [border_color.r, border_color.g, border_color.b, 1.0]
            }
            _ => [0.0; 4],
        }
    }
}
