use crate::{
    Error, Result,
    graphics::{Shader, ShaderSource, Vertex},
};
use std::{num::NonZeroU32, ptr::null_mut, rc::Rc};
use win32::{ComPtr, d3d11::ID3D11Device, try_hresult};

impl Shader {
    /// Create a new [`Shader`]
    pub(in crate::graphics) fn new(
        id: NonZeroU32,
        compiled_shader: &ShaderSource,
        device: &ID3D11Device,
    ) -> Result<Rc<Self>> {
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
        let input_layout = ComPtr::new_in(|input_layout| {
            try_hresult!(device.create_input_layout(
                Vertex::INPUT_LAYOUT.as_ptr(),
                Vertex::INPUT_LAYOUT.len() as _,
                compiled_shader.vertex_content().as_ptr().cast(),
                compiled_shader.vertex_content().len() as _,
                input_layout
            ))
        })
        .map_err(|error| Error::new_inner("unable to create input layout", error))?;

        Ok(Rc::new(Shader {
            id,
            vertex_shader,
            pixel_shader,
            input_layout,
        }))
    }
}
