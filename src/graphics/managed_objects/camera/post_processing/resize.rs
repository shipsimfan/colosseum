use crate::{
    Result,
    graphics::{
        AntiAliasing, CameraPostProcessing,
        managed_objects::camera::post_processing::RenderScaleObjects,
    },
    math::Vector2u,
};
use win32::d3d11::{D3D11_VIEWPORT, ID3D11Device};

impl CameraPostProcessing {
    /// Resizes all render-scale based elements based on `size`
    pub(in crate::graphics::managed_objects::camera) fn resize(
        &mut self,
        window_size: Vector2u,
        relative_viewport: &D3D11_VIEWPORT,
        render_scale: f32,
        anti_aliasing: Option<AntiAliasing>,
        device: &ID3D11Device,
    ) -> Result<()> {
        self.render_scale_objects = RenderScaleObjects::new(
            window_size,
            relative_viewport,
            render_scale,
            anti_aliasing,
            device,
        )?;
        Ok(())
    }
}
