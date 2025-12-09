use crate::{Error, Result, graphics::util::VertexShader};
use std::ptr::null_mut;
use win32::{
    ComPtr,
    d3d11::{D3D11_INPUT_ELEMENT_DESC, ID3D11Device},
    try_hresult,
};

impl VertexShader {
    /// Create a new [`VertexShader`]
    pub fn new(
        compiled_shader: &[u8],
        input_layout: &[D3D11_INPUT_ELEMENT_DESC],
        device: &ID3D11Device,
    ) -> Result<Self> {
        // Create vertex shader
        let shader = ComPtr::new_in(|vertex_shader| {
            try_hresult!(device.create_vertex_shader(
                compiled_shader.as_ptr().cast(),
                compiled_shader.len() as _,
                null_mut(),
                vertex_shader
            ))
        })
        .map_err(|error| Error::new_inner("unable to create vertex shader", error))?;

        // Create input layout
        let input_layout = ComPtr::new_in(|input_layout_ptr| {
            try_hresult!(device.create_input_layout(
                input_layout.as_ptr(),
                input_layout.len() as _,
                compiled_shader.as_ptr().cast(),
                compiled_shader.len() as _,
                input_layout_ptr
            ))
        })
        .map_err(|error| Error::new_inner("unable to create input layout", error))?;

        Ok(VertexShader {
            shader,
            input_layout,
        })
    }
}
