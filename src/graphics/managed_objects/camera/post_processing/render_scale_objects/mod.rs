use crate::graphics::{
    AntiAliasing,
    util::{DepthTexture, RenderTargetTexture},
};
use win32::d3d11::D3D11_VIEWPORT;

mod bind;
mod clear;
mod new;

pub(in crate::graphics) struct RenderScaleObjects {
    /// The first double buffer for the main color render and post processing passes
    pub hdr_output1: RenderTargetTexture,

    /// The second double buffer for post processing passes
    pub hdr_output2: RenderTargetTexture,

    /// The input texture for anti-aliasing
    pub anti_aliasing_input: Option<RenderTargetTexture>,

    /// The input texture for handling render scale
    pub render_scale_input: Option<RenderTargetTexture>,

    /// The texture for recording depth information during the render pass
    pub depth_buffer: DepthTexture,

    /// The region of the internal render target that this camera displays to, in pixels
    pub render_scale_viewport: D3D11_VIEWPORT,

    /// The region of the window that this camera displays to, in pixels
    pub screen_viewport: D3D11_VIEWPORT,

    /// The type of anti-aliasing being used
    pub anti_aliasing: Option<AntiAliasing>,
}
