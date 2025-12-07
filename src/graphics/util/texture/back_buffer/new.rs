use crate::{Error, Result, graphics::util::BackBufferTexture};
use win32::{
    ComInterface, ComPtr,
    d3d11::{
        D3D11_RENDER_TARGET_VIEW_DESC, D3D11_RTV_DIMENSION, D3D11_TEX2D_RTV, ID3D11Device,
        ID3D11Texture2D,
    },
    dxgi::{DXGI_FORMAT, IDXGISwapChain},
    try_hresult,
};

impl BackBufferTexture {
    /// Create a new [`BackBufferTexture`]
    pub fn new(
        format: DXGI_FORMAT,
        swapchain: &mut IDXGISwapChain,
        device: &ID3D11Device,
    ) -> Result<Self> {
        // Get the back buffer
        let mut texture = ComPtr::<ID3D11Texture2D>::new_in(|texture| {
            try_hresult!(swapchain.get_buffer(0, &ID3D11Texture2D::IID, texture.cast()))
        })
        .map_err(|os| Error::new_inner("unable to get swapchain back buffer", os))?;

        // Create render target view
        let view = ComPtr::new_in(|view| {
            try_hresult!(device.create_render_target_view(
                texture.as_mut(),
                &D3D11_RENDER_TARGET_VIEW_DESC {
                    format,
                    view_dimension: D3D11_RTV_DIMENSION::Texture2D,
                    u: win32::d3d11::D3D11_RENDER_TARGET_VIEW_DESC_UNION {
                        texture_2d: D3D11_TEX2D_RTV { mip_slice: 0 }
                    }
                },
                view
            ))
        })
        .map_err(|os| Error::new_inner("unable to create back buffer render target view", os))?;

        Ok(BackBufferTexture { view })
    }
}
