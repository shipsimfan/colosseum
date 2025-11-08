use crate::{
    Error, Result,
    graphics::context::{DEPTH_FORMAT, RENDER_TARGET_FORMAT, SwapchainObjects},
    math::Vector2u,
};
use std::ptr::null;
use win32::{
    ComInterface, ComPtr,
    d3d11::{
        D3D11_BIND_FLAG, D3D11_DEPTH_STENCIL_VIEW_DESC, D3D11_DEPTH_STENCIL_VIEW_DESC_UNION,
        D3D11_DSV_DIMENSION, D3D11_RENDER_TARGET_VIEW_DESC, D3D11_RTV_DIMENSION, D3D11_TEX2D_DSV,
        D3D11_TEX2D_RTV, D3D11_TEXTURE2D_DESC, D3D11_USAGE, D3D11_VIEWPORT, ID3D11Device,
        ID3D11Resource,
    },
    dxgi::{DXGI_SAMPLE_DESC, IDXGISwapChain},
    try_hresult,
};

impl SwapchainObjects {
    /// Create new [`SwapchainObjects`]
    pub fn new(
        swapchain: &mut IDXGISwapChain,
        swapchain_size: Vector2u,
        device: &ID3D11Device,
    ) -> Result<Self> {
        // Get the back buffer
        let mut back_buffer = ComPtr::<ID3D11Resource>::new_in(|ptr| {
            try_hresult!(swapchain.get_buffer(0, &ID3D11Resource::IID, ptr.cast()))
        })
        .map_err(|os| Error::new_inner("unable to get swapchain back buffer", os))?;

        // Create render target view
        let render_target_view = ComPtr::new_in(|ptr| {
            try_hresult!(device.create_render_target_view(
                back_buffer.as_mut(),
                &D3D11_RENDER_TARGET_VIEW_DESC {
                    format: RENDER_TARGET_FORMAT,
                    view_dimension: D3D11_RTV_DIMENSION::Texture2D,
                    u: win32::d3d11::D3D11_RENDER_TARGET_VIEW_DESC_UNION {
                        texture_2d: D3D11_TEX2D_RTV { mip_slice: 0 }
                    }
                },
                ptr
            ))
        })
        .map_err(|os| Error::new_inner("unable to create render target view", os))?;

        // Create depth buffer
        let depth_buffer_desc = D3D11_TEXTURE2D_DESC {
            width: swapchain_size.x,
            height: swapchain_size.y,
            mip_levels: 1,
            array_size: 1,
            format: DEPTH_FORMAT,
            sample_desc: DXGI_SAMPLE_DESC {
                count: 1,
                quality: 0,
            },
            usage: D3D11_USAGE::Default,
            bind_flags: D3D11_BIND_FLAG::DepthStencil as _,
            cpu_access_flags: 0,
            misc_flags: 0,
        };
        let mut depth_buffer = ComPtr::new_in(|depth_buffer| {
            try_hresult!(device.create_texture_2d(&depth_buffer_desc, null(), depth_buffer))
        })
        .map_err(|error| Error::new_inner("unable to create depth buffer", error))?;

        // Create depth buffer view
        let depth_stencil_view_desc = D3D11_DEPTH_STENCIL_VIEW_DESC {
            format: DEPTH_FORMAT,
            view_dimension: D3D11_DSV_DIMENSION::Texture2D,
            flags: 0,
            u: D3D11_DEPTH_STENCIL_VIEW_DESC_UNION {
                texture_2d: D3D11_TEX2D_DSV { mip_slice: 0 },
            },
        };
        let depth_stencil_view = ComPtr::new_in(|depth_stencil_view| {
            try_hresult!(device.create_depth_stencil_view(
                depth_buffer.as_mut(),
                &depth_stencil_view_desc,
                depth_stencil_view
            ))
        })
        .map_err(|error| Error::new_inner("unable to create depth stencil view", error))?;

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
            viewport,
            depth_buffer,
            depth_stencil_view,
            back_buffer,
            render_target_view,
        })
    }
}
