use crate::{
    Result,
    graphics::{
        AmbientLight, managed_objects::lights::ambient::LightCbContent, util::ConstantBuffer,
    },
};
use win32::d3d11::ID3D11Device;

impl AmbientLight {
    /// Create a new [`AmbientLight`]
    pub(in crate::graphics::managed_objects::lights) fn new(device: &ID3D11Device) -> Result<Self> {
        Ok(AmbientLight {
            constant_buffer: ConstantBuffer::new(
                LightCbContent::default(),
                ConstantBuffer::LIGHTING_SLOT,
                device,
            )?,
        })
    }
}
