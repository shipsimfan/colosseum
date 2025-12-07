use crate::{
    Result,
    graphics::{PointLights, managed_objects::lights::LightList},
};
use std::num::NonZeroUsize;
use win32::d3d11::ID3D11Device;

impl PointLights {
    /// Create a new empty set of [`PointLights`]
    pub(in crate::graphics::managed_objects::lights) fn new(device: &ID3D11Device) -> Result<Self> {
        Ok(PointLights {
            list: LightList::new(NonZeroUsize::new(16).unwrap(), 1, device)?,
        })
    }
}
