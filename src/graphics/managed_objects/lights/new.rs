use crate::{
    Result,
    graphics::{AmbientLight, DirectionalLights, Lights, PointLights, SpotLights},
};
use win32::d3d11::ID3D11Device;

impl Lights {
    /// Create a new set of [`Lights`]
    pub fn new(device: &ID3D11Device) -> Result<Self> {
        Ok(Lights {
            ambient: AmbientLight::new(device)?,
            directional: DirectionalLights::new(device)?,
            point: PointLights::new(device)?,
            spot: SpotLights::new(device)?,
        })
    }
}
