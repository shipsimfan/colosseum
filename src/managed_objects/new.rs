use crate::{
    ManagedObjects, Result, Transforms,
    graphics::{ManagedGraphicsObjects, Shader},
};
use win32::{ComPtr, d3d11::ID3D11Device};

impl ManagedObjects {
    /// Create a new set of [`ManagedObjects`]
    pub(crate) fn new(
        default_lit_shader: Shader,
        default_unlit_shader: Shader,
        device: &ComPtr<ID3D11Device>,
    ) -> Result<Self> {
        Ok(ManagedObjects {
            graphics: ManagedGraphicsObjects::new(
                default_lit_shader,
                default_unlit_shader,
                device,
            )?,
            transforms: Transforms::new(),
        })
    }
}
