use crate::{
    Result,
    graphics::{PostProcessing, context::post_processing::RenderScaleObjects},
    math::Vector2u,
};
use win32::d3d11::ID3D11Device;

impl PostProcessing {
    /// Resizes all render-scale based elements based on `size`
    pub(in crate::graphics::context) fn resize(
        &mut self,
        size: Vector2u,
        device: &ID3D11Device,
    ) -> Result<()> {
        self.render_scale_objects = RenderScaleObjects::new(
            size,
            self.render_scale,
            self.anti_aliasing.is_some(),
            device,
        )?;
        Ok(())
    }
}
