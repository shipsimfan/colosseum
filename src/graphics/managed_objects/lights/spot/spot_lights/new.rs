use crate::{
    Result,
    graphics::{SpotLights, managed_objects::lights::LightList},
};
use std::num::NonZeroUsize;
use win32::d3d11::ID3D11Device;

impl SpotLights {
    /// Create a new empty set of [`SpotLights`]
    pub(in crate::graphics::managed_objects::lights) fn new(device: &ID3D11Device) -> Result<Self> {
        Ok(SpotLights {
            list: LightList::new(NonZeroUsize::new(4).unwrap(), 2, device)?,
        })
    }
}
