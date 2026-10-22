use std::marker::PhantomData;

mod add;
mod create_resources;
mod node;
mod system;

/// Performs color correction, tone mapping, gamma correction, and color grading
#[derive(Debug)]
pub(in crate::render::frame_graph) struct ToneMapNode<Game: crate::Game> {
    /// The index of the descriptor set used for the input image
    descriptor_set: usize,

    /// A marker for the game type
    _marker: PhantomData<Game>,
}

#[repr(C)]
struct PushConstants {
    contrast: f32,
    saturation: f32,
    exposure: f32,
    gamma: f32,
}
