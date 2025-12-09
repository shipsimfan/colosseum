use crate::{Error, Result, graphics::util::PixelShader};
use std::ptr::null_mut;
use win32::{ComPtr, d3d11::ID3D11Device, try_hresult};

impl PixelShader {
    /// Create a new [`PixelShader`]
    pub fn new(compiled_shader: &[u8], device: &ID3D11Device) -> Result<Self> {
        let shader = ComPtr::new_in(|pixel_shader| {
            try_hresult!(device.create_pixel_shader(
                compiled_shader.as_ptr().cast(),
                compiled_shader.len() as _,
                null_mut(),
                pixel_shader
            ))
        })
        .map_err(|error| Error::new_inner("unable to create pixel shader", error))?;

        Ok(PixelShader { shader })
    }
}
