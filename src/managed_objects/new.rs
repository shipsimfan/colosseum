use crate::{ManagedObjects, Result, Transforms, graphics::ManagedGraphicsObjects};
use win32::{ComPtr, d3d11::ID3D11Device};

impl ManagedObjects {
    /// Create a new set of [`ManagedObjects`]
    pub(crate) fn new(device: &ComPtr<ID3D11Device>) -> Result<Self> {
        Ok(ManagedObjects {
            graphics: ManagedGraphicsObjects::new(device)?,
            transforms: Transforms::new(),
        })
    }
}
