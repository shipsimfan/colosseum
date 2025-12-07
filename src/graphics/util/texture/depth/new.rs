use crate::{Error, Result, graphics::util::DepthTexture, math::Vector2u};
use std::ptr::null;
use win32::{
    ComPtr,
    d3d11::{
        D3D11_BIND_FLAG, D3D11_DEPTH_STENCIL_VIEW_DESC, D3D11_DEPTH_STENCIL_VIEW_DESC_UNION,
        D3D11_DSV_DIMENSION, D3D11_TEX2D_DSV, D3D11_TEXTURE2D_DESC, D3D11_USAGE, ID3D11Device,
    },
    dxgi::{DXGI_FORMAT, DXGI_SAMPLE_DESC},
    try_hresult,
};

const DEPTH_FORMAT: DXGI_FORMAT = DXGI_FORMAT::D32Float;

impl DepthTexture {
    /// Create a new [`DepthTexture`]
    pub fn new(size: Vector2u, device: &ID3D11Device) -> Result<Self> {
        // Create depth buffer
        let buffer_desc = D3D11_TEXTURE2D_DESC {
            width: size.x,
            height: size.y,
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
        let mut buffer = ComPtr::new_in(|buffer| {
            try_hresult!(device.create_texture_2d(&buffer_desc, null(), buffer))
        })
        .map_err(|error| Error::new_inner("unable to create depth buffer", error))?;

        // Create depth buffer view
        let view_desc = D3D11_DEPTH_STENCIL_VIEW_DESC {
            format: DEPTH_FORMAT,
            view_dimension: D3D11_DSV_DIMENSION::Texture2D,
            flags: 0,
            u: D3D11_DEPTH_STENCIL_VIEW_DESC_UNION {
                texture_2d: D3D11_TEX2D_DSV { mip_slice: 0 },
            },
        };
        let view = ComPtr::new_in(|view| {
            try_hresult!(device.create_depth_stencil_view(buffer.as_mut(), &view_desc, view))
        })
        .map_err(|error| Error::new_inner("unable to create depth stencil view", error))?;

        Ok(DepthTexture { view })
    }
}
