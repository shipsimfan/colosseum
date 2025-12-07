use crate::{graphics::Camera, util::Arena};
use win32::{ComPtr, d3d11::ID3D11Device};

mod create;
mod get;
mod index;
mod iter;
mod new;
mod remove;

/// A set of [`Camera`]s registered with the engine
pub struct Cameras {
    /// The set of [`Camera`]s
    arena: Arena<Camera>,

    /// The device for creating new [`Camera`]s
    device: ComPtr<ID3D11Device>,
}
