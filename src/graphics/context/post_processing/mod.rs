use crate::graphics::util::{DepthTexture, RenderTargetTexture};
use win32::d3d11::D3D11_VIEWPORT;

mod aa;

mod bind;
mod clear;
mod new;
mod run;

pub use aa::AntiAliasing;

/// The objects used during post processing
pub(in crate::graphics::context) struct PostProcessing {
    /// The first double buffer for the main color render and post processing passes
    hdr_output1: RenderTargetTexture,

    /// The second double buffer for post processing passes
    hdr_output2: RenderTargetTexture,

    /// The type of anti-aliasing and the texture it reads from
    anti_aliasing: Option<(AntiAliasing, RenderTargetTexture)>,

    /// The second LDR texture for the output if render scale is active
    render_scale_input: Option<RenderTargetTexture>,

    /// The texture for recording depth information during the render pass
    depth_buffer: DepthTexture,

    /// The viewport to render objects into
    viewport: D3D11_VIEWPORT,
}
