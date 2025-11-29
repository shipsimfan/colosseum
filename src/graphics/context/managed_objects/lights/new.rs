use crate::{
    Result,
    graphics::context::{Lights, managed_objects::lights::LightConstantBuffer},
};
use win32::d3d11::ID3D11Device;

impl Lights {
    /// Create a new set of [`Lights`]
    pub(in crate::graphics::context::managed_objects) fn new(
        device: &ID3D11Device,
    ) -> Result<Self> {
        Ok(Lights {
            constant_buffer: LightConstantBuffer::new(device)?,
        })
    }
}
