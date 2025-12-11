use crate::{
    graphics::{AntiAliasing, managed_objects::camera::post_processing::RenderScaleObjects},
    math::Vector2u,
};

impl RenderScaleObjects {
    /// Get the type of anti-aliasing in use
    pub fn anti_aliasing(&self) -> Option<AntiAliasing> {
        self.anti_aliasing
    }

    /// Get the size the objects for rendering are
    pub fn render_size(&self) -> Vector2u {
        self.render_size
    }
}
