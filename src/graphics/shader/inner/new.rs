use crate::{
    Error, Result,
    graphics::{ShaderInner, ShaderSource, Vertex},
};
use std::{num::NonZeroU32, ptr::null_mut};
use win32::{
    ComPtr,
    d3d11::{D3D11_INPUT_ELEMENT_DESC, ID3D11Device},
    try_hresult,
};

impl ShaderInner {
    /// Create a new unlit [`ShaderInner`]
    pub(in crate::graphics::shader) fn new_unlit(
        id: NonZeroU32,
        compiled_shader: &ShaderSource,
        device: &ID3D11Device,
    ) -> Result<Self> {
        Self::new(id, compiled_shader, device, Vertex::UNLIT_INPUT_LAYOUT)
    }

    /// Create a new lit [`ShaderInner`]
    pub(in crate::graphics::shader) fn new_lit(
        id: NonZeroU32,
        compiled_shader: &ShaderSource,
        device: &ID3D11Device,
    ) -> Result<Self> {
        Self::new(id, compiled_shader, device, Vertex::LIT_INPUT_LAYOUT)
    }

    /// Create a new [`ShaderInner`]
    fn new(
        id: NonZeroU32,
        compiled_shader: &ShaderSource,
        device: &ID3D11Device,
        input_layout: &[D3D11_INPUT_ELEMENT_DESC],
    ) -> Result<Self> {
        // Create vertex shader
        let vertex_shader = ComPtr::new_in(|vertex_shader| {
            try_hresult!(device.create_vertex_shader(
                compiled_shader.vertex_content().as_ptr().cast(),
                compiled_shader.vertex_content().len() as _,
                null_mut(),
                vertex_shader
            ))
        })
        .map_err(|error| Error::new_inner("unable to create vertex shader", error))?;

        // Create pixel shader
        let pixel_shader = ComPtr::new_in(|pixel_shader| {
            try_hresult!(device.create_pixel_shader(
                compiled_shader.pixel_content().as_ptr().cast(),
                compiled_shader.pixel_content().len() as _,
                null_mut(),
                pixel_shader
            ))
        })
        .map_err(|error| Error::new_inner("unable to create pixel shader", error))?;

        // Create input layout
        let input_layout = ComPtr::new_in(|input_layout_ptr| {
            try_hresult!(device.create_input_layout(
                input_layout.as_ptr(),
                input_layout.len() as _,
                compiled_shader.vertex_content().as_ptr().cast(),
                compiled_shader.vertex_content().len() as _,
                input_layout_ptr
            ))
        })
        .map_err(|error| Error::new_inner("unable to create input layout", error))?;

        Ok(ShaderInner {
            id,
            vertex_shader,
            pixel_shader,
            input_layout,
        })
    }
}
