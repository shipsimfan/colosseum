use crate::{
    Result,
    graphics::{Cameras, Lights, ManagedGraphicsObjects, Materials, MeshRenderers},
};
use win32::{ComPtr, d3d11::ID3D11Device};

impl ManagedGraphicsObjects {
    /// Create a new set of [`ManagedGraphicsObjects`]
    pub(crate) fn new(device: &ComPtr<ID3D11Device>) -> Result<Self> {
        Ok(ManagedGraphicsObjects {
            cameras: Cameras::new(device.clone()),
            opaque_materials: Materials::new(device.clone())?,
            mesh_renderers: MeshRenderers::new(device.clone()),
            lights: Lights::new(device)?,
        })
    }
}
