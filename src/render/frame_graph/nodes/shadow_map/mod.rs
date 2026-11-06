use crate::render::ShadowMapLight;
use std::marker::PhantomData;

mod add;
mod create_resources;
mod node;
mod system;

/// Renders shadow maps for lights of a specific type
#[derive(Debug)]
pub(in crate::render::frame_graph) struct ShadowMapNode<L: ShadowMapLight> {
    /// The index of the descriptor set used for the shadow map
    descriptor_set: usize,

    /// A marker for the type of light this shadow map node handles
    _light: PhantomData<fn() -> L>,
}

#[repr(C)]
struct PushConstants {
    light_index: u32,
    object_index: u32,
}
