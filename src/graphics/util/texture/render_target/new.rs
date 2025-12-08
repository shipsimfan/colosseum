use crate::{Error, Result, graphics::util::RenderTargetTexture, math::Vector2u};
use std::ptr::null;
use win32::{
    ComPtr,
    d3d11::{
        D3D11_BIND_FLAG, D3D11_RENDER_TARGET_VIEW_DESC, D3D11_RENDER_TARGET_VIEW_DESC_UNION,
        D3D11_RTV_DIMENSION, D3D11_SHADER_RESOURCE_VIEW_DESC,
        D3D11_SHADER_RESOURCE_VIEW_DESC_UNION, D3D11_SRV_DIMENSION, D3D11_TEX2D_RTV,
        D3D11_TEX2D_SRV, D3D11_TEXTURE2D_DESC, D3D11_USAGE, ID3D11Device,
    },
    dxgi::{DXGI_FORMAT, DXGI_SAMPLE_DESC},
    try_hresult,
};

impl RenderTargetTexture {
    /// Create a new [`RenderTargetTexture`]
    pub fn new(
        size: Vector2u,
        format: DXGI_FORMAT,
        shader_slot: u32,
        device: &ID3D11Device,
    ) -> Result<Self> {
        // Create texture
        let texture_desc = D3D11_TEXTURE2D_DESC {
            width: size.x,
            height: size.y,
            mip_levels: 1,
            array_size: 1,
            format,
            sample_desc: DXGI_SAMPLE_DESC {
                count: 1,
                quality: 0,
            },
            usage: D3D11_USAGE::Default,
            bind_flags: D3D11_BIND_FLAG::RenderTarget as u32
                | D3D11_BIND_FLAG::ShaderResource as u32,
            cpu_access_flags: 0,
            misc_flags: 0,
        };

        let mut texture = ComPtr::new_in(|texture| {
            try_hresult!(device.create_texture_2d(&texture_desc, null(), texture))
        })
        .map_err(|error| Error::new_inner("unable to create a render target texture", error))?;

        // Create render view
        let render_view_desc = D3D11_RENDER_TARGET_VIEW_DESC {
            format,
            view_dimension: D3D11_RTV_DIMENSION::Texture2D,
            u: D3D11_RENDER_TARGET_VIEW_DESC_UNION {
                texture_2d: D3D11_TEX2D_RTV { mip_slice: 0 },
            },
        };

        let render_view = ComPtr::new_in(|render_view| {
            try_hresult!(device.create_render_target_view(
                texture.as_mut(),
                &render_view_desc,
                render_view
            ))
        })
        .map_err(|error| {
            Error::new_inner("unable to create a render target texture view", error)
        })?;

        // Create shader view
        let shader_view_desc = D3D11_SHADER_RESOURCE_VIEW_DESC {
            format,
            view_dimension: D3D11_SRV_DIMENSION::Texture2D,
            u: D3D11_SHADER_RESOURCE_VIEW_DESC_UNION {
                texture_2d: D3D11_TEX2D_SRV {
                    most_detailed_mip: 0,
                    mip_levels: u32::MAX,
                },
            },
        };

        let shader_view = ComPtr::new_in(|shader_view| {
            try_hresult!(device.create_shader_resource_view(
                texture.as_mut(),
                &shader_view_desc,
                shader_view
            ))
        })
        .map_err(|error| {
            Error::new_inner(
                "unable to create a render target texture shader view",
                error,
            )
        })?;

        Ok(RenderTargetTexture {
            render_view,
            shader_view,
            shader_slot,
        })
    }
}
