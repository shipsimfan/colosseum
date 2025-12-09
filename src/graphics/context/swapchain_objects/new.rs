use crate::{
    Result,
    graphics::{
        context::{SWAPCHAIN_FORMAT, SwapchainObjects},
        util::BackBufferTexture,
    },
};
use win32::{d3d11::ID3D11Device, dxgi::IDXGISwapChain};

impl SwapchainObjects {
    /// Create new [`SwapchainObjects`]
    pub(in crate::graphics::context) fn new(
        swapchain: &mut IDXGISwapChain,
        device: &ID3D11Device,
    ) -> Result<Self> {
        let back_buffer = BackBufferTexture::new(SWAPCHAIN_FORMAT, swapchain, device)?;

        Ok(SwapchainObjects { back_buffer })
    }
}
