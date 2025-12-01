use crate::{Error, Result, graphics::context::managed_objects::lights::LightConstantBuffer};
use win32::{
    d3d11::{D3D11_MAP, D3D11_MAPPED_SUBRESOURCE, ID3D11DeviceContext},
    try_hresult,
};

impl LightConstantBuffer {
    /// Bind the lights constant buffer
    pub fn bind(
        &mut self,
        num_directional_lights: Option<usize>,
        num_point_lights: Option<usize>,
        num_spot_lights: Option<usize>,
        device_context: &mut ID3D11DeviceContext,
    ) -> Result<()> {
        if let Some(num_directional_lights) = num_directional_lights {
            self.content.num_directional_lights = num_directional_lights as _;
            self.dirty = true;
        }

        if let Some(num_point_lights) = num_point_lights {
            self.content.num_point_lights = num_point_lights as _;
            self.dirty = true;
        }

        if let Some(num_spot_lights) = num_spot_lights {
            self.content.num_spot_lights = num_spot_lights as _;
            self.dirty = true;
        }

        // Update constant buffer if needed
        if self.dirty {
            let mut mapped_resource = D3D11_MAPPED_SUBRESOURCE::default();
            try_hresult!(device_context.map(
                self.buffer.as_mut(),
                0,
                D3D11_MAP::WriteDiscard,
                0,
                &mut mapped_resource,
            ))
            .map_err(|error| Error::new_inner("unable to map light constant buffer", error))?;

            *unsafe { &mut *(mapped_resource.data as *mut _) } = self.content;

            device_context.unmap(self.buffer.as_mut(), 0);
            self.dirty = false;
        }

        // Bind the buffer
        let buffer = self.buffer.as_mut() as *mut _;
        device_context.vs_set_constant_buffers(2, 1, &buffer);
        device_context.ps_set_constant_buffers(2, 1, &buffer);

        Ok(())
    }
}
