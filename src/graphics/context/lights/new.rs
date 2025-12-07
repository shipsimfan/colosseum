use crate::{
    Result,
    graphics::{
        context::{
            Lights,
            lights::{LightCbContent, LightList},
        },
        util::ConstantBuffer,
    },
};
use std::num::NonZeroUsize;
use win32::d3d11::ID3D11Device;

impl Lights {
    /// Create a new set of [`Lights`]
    pub(in crate::graphics::context) fn new(device: &ID3D11Device) -> Result<Self> {
        Ok(Lights {
            constant_buffer: ConstantBuffer::new(LightCbContent::default(), 2, device)?,
            directional_lights: LightList::new(NonZeroUsize::new(1).unwrap(), 0, device)?,
            point_lights: LightList::new(NonZeroUsize::new(16).unwrap(), 1, device)?,
            spot_lights: LightList::new(NonZeroUsize::new(4).unwrap(), 2, device)?,
        })
    }
}
