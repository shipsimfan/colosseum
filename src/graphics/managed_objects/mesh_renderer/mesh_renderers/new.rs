use crate::{graphics::MeshRenderers, util::Arena};
use win32::{ComPtr, d3d11::ID3D11Device};

impl MeshRenderers {
    /// Create a new empty set of [`MeshRenderers`]
    pub(in crate::graphics::managed_objects) fn new(device: ComPtr<ID3D11Device>) -> Self {
        MeshRenderers {
            arena: Arena::new(),
            device,
        }
    }
}
