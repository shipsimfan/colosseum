use crate::{
    Result,
    graphics::{
        context::{SWAPCHAIN_FORMAT, SwapchainObjects},
        util::BackBufferTexture,
    },
    math::Vector2u,
};
use win32::{
    d3d11::{D3D11_VIEWPORT, ID3D11Device},
    dxgi::IDXGISwapChain,
};

impl SwapchainObjects {
    /// Create new [`SwapchainObjects`]
    pub fn new(
        swapchain: &mut IDXGISwapChain,
        swapchain_size: Vector2u,
        device: &ID3D11Device,
    ) -> Result<Self> {
        let back_buffer = BackBufferTexture::new(SWAPCHAIN_FORMAT, swapchain, device)?;

        // Create viewport
        let viewport = D3D11_VIEWPORT {
            top_left_x: 0.0,
            top_left_y: 0.0,
            width: swapchain_size.x as _,
            height: swapchain_size.y as _,
            min_depth: 0.0,
            max_depth: 1.0,
        };

        Ok(SwapchainObjects {
            back_buffer,
            viewport,
        })
    }
}
