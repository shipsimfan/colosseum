use crate::{
    Result,
    graphics::{
        AntiAliasing, CameraPostProcessing,
        managed_objects::camera::post_processing::RenderScaleObjects,
    },
    math::Vector2u,
    util::Arena,
};
use win32::d3d11::{D3D11_VIEWPORT, ID3D11Device};

impl CameraPostProcessing {
    /// Create a new [`PostProcessing`] context
    pub(in crate::graphics::managed_objects::camera) fn new(
        window_size: Vector2u,
        relative_viewport: &D3D11_VIEWPORT,
        render_scale: f32,
        anti_aliasing: Option<AntiAliasing>,
        device: &ID3D11Device,
    ) -> Result<Self> {
        // Create render scale objects
        let render_scale_objects = RenderScaleObjects::new(
            window_size,
            relative_viewport,
            render_scale,
            anti_aliasing,
            device,
        )?;

        Ok(CameraPostProcessing {
            provided_post_processing: Arena::new(),
            render_scale_objects,
        })
    }
}
