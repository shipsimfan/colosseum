use crate::{graphics::Cameras, util::Arena};
use win32::{ComPtr, d3d11::ID3D11Device};

impl Cameras {
    /// Create a new empty set of [`Cameras`]
    pub(in crate::graphics::managed_objects) fn new(device: ComPtr<ID3D11Device>) -> Self {
        Cameras {
            arena: Arena::new(),
            device,
        }
    }
}
