use crate::{
    Result,
    graphics::{
        context::post_processing::render_scale_objects::RenderScaleObjects,
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
    pub fn new(
        size: Vector2u,
        render_scale: f32,
        anti_aliasing: bool,
        device: &ID3D11Device,
    ) -> Result<Self> {
        // Calculate render scale
        let size = Vector2u::new(
            (size.x as f32 * render_scale) as u32,
            (size.y as f32 * render_scale) as u32,
        );

        // Create outputs for provided post-processing stages
        let hdr_output1 = RenderTargetTexture::new(size, HDR_FORMAT, 0, device)?;
        let hdr_output2 = RenderTargetTexture::new(size, HDR_FORMAT, 0, device)?;

        // Create anti-aliasing and render-scale targets
        let anti_aliasing_input = if anti_aliasing {
            Some(RenderTargetTexture::new(size, LDR_FORMAT, 0, device)?)
        } else {
            None
        };
        let render_scale_input = if render_scale == 1.0 {
            None
        } else {
            Some(RenderTargetTexture::new(size, LDR_FORMAT, 0, device)?)
        };

        // Create depth buffer
        let depth_buffer = DepthTexture::new(size, device)?;

        // Create viewport
        let viewport = D3D11_VIEWPORT {
            top_left_x: 0.0,
            top_left_y: 0.0,
            width: size.x as _,
            height: size.y as _,
            min_depth: 0.0,
            max_depth: 1.0,
        };

        Ok(RenderScaleObjects {
            hdr_output1,
            hdr_output2,
            anti_aliasing_input,
            render_scale_input,
            depth_buffer,
            viewport,
        })
    }
}
