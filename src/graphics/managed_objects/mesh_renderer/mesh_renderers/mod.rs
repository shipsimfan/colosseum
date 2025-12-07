use crate::{graphics::MeshRenderer, util::Arena};
use win32::{ComPtr, d3d11::ID3D11Device};

mod clear;
mod create;
mod get;
mod index;
mod iter;
mod new;
mod remove;

/// A set of [`MeshRenderer`]s registered with the engine
pub struct MeshRenderers {
    /// The set of [`MeshRenderer`]s
    arena: Arena<MeshRenderer>,

    /// The device for creating new [`MeshRenderer`]s
    device: ComPtr<ID3D11Device>,
}
