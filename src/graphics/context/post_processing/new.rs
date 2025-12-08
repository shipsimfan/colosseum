use crate::{
    Result,
    graphics::{
        AntiAliasing,
        context::{PostProcessing, post_processing::RenderScaleObjects},
    },
    math::Vector2u,
    util::Arena,
};
use win32::{d3d11::ID3D11Device, dxgi::DXGI_FORMAT};

const HDR_FORMAT: DXGI_FORMAT = DXGI_FORMAT::R16G16B16A16Float;
const LDR_FORMAT: DXGI_FORMAT = DXGI_FORMAT::R8G8B8A8UNorm;

impl PostProcessing {
    /// Create a new [`PostProcessing`] context
    pub(in crate::graphics::context) fn new(
        size: Vector2u,
        render_scale: f32,
        anti_aliasing: Option<AntiAliasing>,
        device: &ID3D11Device,
    ) -> Result<Self> {
        let render_scale_objects =
            RenderScaleObjects::new(size, render_scale, anti_aliasing.is_some(), device)?;

        Ok(PostProcessing {
            provided_post_processing: Arena::new(),
            render_scale_objects,
            render_scale,
            anti_aliasing,
        })
    }
}
