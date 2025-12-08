use crate::{
    Result,
    graphics::{
        context::{AntiAliasing, PostProcessing},
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

impl PostProcessing {
    /// Create a new set of [`PostProcessing`] objects
    pub fn new(
        size: Vector2u,
        render_scale: f32,
        anti_aliasing: Option<AntiAliasing>,
        device: &ID3D11Device,
    ) -> Result<Self> {
        let size = Vector2u::new(
            (size.x as f32 * render_scale) as u32,
            (size.y as f32 * render_scale) as u32,
        );

        let hdr_output1 = RenderTargetTexture::new(size, HDR_FORMAT, 0, device)?;
        let hdr_output2 = RenderTargetTexture::new(size, HDR_FORMAT, 0, device)?;

        let anti_aliasing = match anti_aliasing {
            Some(anti_aliasing) => Some((
                anti_aliasing,
                RenderTargetTexture::new(size, LDR_FORMAT, 0, device)?,
            )),
            None => None,
        };
        let render_scale_input = if render_scale == 1.0 {
            None
        } else {
            Some(RenderTargetTexture::new(size, LDR_FORMAT, 0, device)?)
        };

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

        Ok(PostProcessing {
            hdr_output1,
            hdr_output2,
            anti_aliasing,
            render_scale_input,
            depth_buffer,
            viewport,
        })
    }
}
