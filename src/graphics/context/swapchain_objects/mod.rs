use crate::graphics::util::BackBufferTexture;

mod get;
mod new;
mod unbind;

/// The objects associated directly with the swapchain that must be re-created when the swapchan is
/// resized
pub(in crate::graphics) struct SwapchainObjects {
    /// The back buffer to render the final image into
    back_buffer: BackBufferTexture,
}
