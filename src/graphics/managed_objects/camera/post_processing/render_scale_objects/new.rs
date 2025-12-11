use crate::{
    Result,
    graphics::{
        AntiAliasing,
        managed_objects::camera::post_processing::RenderScaleObjects,
        util::{DepthTexture, RenderTargetTexture},
    },
    math::Vector2u,
};
use win32::{
    d3d11::{D3D11_VIEWPORT, ID3D11Device},
    dxgi::DXGI_FORMAT,
};

const HDR_FORMAT: DXGI_FORMAT = DXGI_FORMAT::R16G16B16A16Float;
const LDR_FORMAT: DXGI_FORMAT = DXGI_FORMAT::R8G8B8A8UNorm;

impl RenderScaleObjects {
    /// Create a new set of [`RenderScaleObjects`]
    pub(in crate::graphics::managed_objects::camera) fn new(
        window_size: Vector2u,
        relative_viewport: &D3D11_VIEWPORT,
        render_scale: f32,
        anti_aliasing: Option<AntiAliasing>,
        device: &ID3D11Device,
    ) -> Result<Self> {
        // Calculate render scale
        let render_size = Vector2u::new(
            (window_size.x as f32 * render_scale * relative_viewport.width) as u32,
            (window_size.y as f32 * render_scale * relative_viewport.height) as u32,
        );

        // Create outputs for provided post-processing stages
        let hdr_output1 = RenderTargetTexture::new(render_size, HDR_FORMAT, 0, device)?;
        let hdr_output2 = RenderTargetTexture::new(render_size, HDR_FORMAT, 0, device)?;

        // Create anti-aliasing and render-scale targets
        let anti_aliasing_input = if anti_aliasing.is_some() {
            Some(RenderTargetTexture::new(
                render_size,
                LDR_FORMAT,
                0,
                device,
            )?)
        } else {
            None
        };
        let render_scale_input = if render_scale == 1.0 {
            None
        } else {
            Some(RenderTargetTexture::new(
                render_size,
                LDR_FORMAT,
                0,
                device,
            )?)
        };

        // Create depth buffer
        let depth_buffer = DepthTexture::new(render_size, device)?;

        // Create viewports
        let screen_viewport = D3D11_VIEWPORT {
            top_left_x: relative_viewport.top_left_x * window_size.x as f32,
            top_left_y: relative_viewport.top_left_y * window_size.y as f32,
            width: relative_viewport.width * window_size.x as f32,
            height: relative_viewport.height * window_size.y as f32,
            min_depth: 0.0,
            max_depth: 1.0,
        };
        let render_scale_viewport = D3D11_VIEWPORT {
            top_left_x: 0.0,
            top_left_y: 0.0,
            width: render_size.x as _,
            height: render_size.y as _,
            min_depth: 0.0,
            max_depth: 1.0,
        };

        Ok(RenderScaleObjects {
            hdr_output1,
            hdr_output2,
            anti_aliasing_input,
            render_scale_input,
            depth_buffer,
            screen_viewport,
            render_scale_viewport,
            anti_aliasing,
            render_size,
        })
    }
}
