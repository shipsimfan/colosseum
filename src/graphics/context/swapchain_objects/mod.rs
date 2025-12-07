use crate::graphics::util::{BackBufferTexture, DepthTexture};
use win32::d3d11::D3D11_VIEWPORT;

mod bind;
mod clear;
mod new;
mod unbind;

/// The objects associated directly with the swapchain that must be re-created when the swapchan is
/// resized
pub(in crate::graphics::context) struct SwapchainObjects {
    /// The back buffer to render the final image into
    back_buffer: BackBufferTexture,

    /// The texture for recording depth information during the render pass
    depth_buffer: DepthTexture,

    /// The viewport to render objects into
    viewport: D3D11_VIEWPORT,
}
