use crate::{
    Result,
    graphics::{DirectionalLights, managed_objects::lights::LightList},
};
use std::num::NonZeroUsize;
use win32::d3d11::ID3D11Device;

impl DirectionalLights {
    /// Create a new empty set of [`DirectionalLights`]
    pub(in crate::graphics::managed_objects::lights) fn new(device: &ID3D11Device) -> Result<Self> {
        Ok(DirectionalLights {
            list: LightList::new(NonZeroUsize::new(1).unwrap(), 0, device)?,
        })
    }
}
