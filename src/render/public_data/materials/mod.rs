use crate::render::GraphicsDevice;
use std::sync::Arc;

mod id;
mod lit_forward;
mod unlit;

mod create;
mod get;
mod new;

pub use id::*;
pub use lit_forward::*;
pub use unlit::*;

/// The collection of materials available for rendering
pub struct Materials {
    /// The unlit materials available for rendering
    unlit: UnlitMaterials,

    /// The lit forward materials available for rendering
    lit_forward: LitForwardMaterials,

    /// The device for creating pipelines
    device: Arc<GraphicsDevice>,
}
