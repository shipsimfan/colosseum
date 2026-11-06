mod add;
mod node;
mod system;

/// Renders lit objects using a single forward rendering pass
#[derive(Debug)]
pub(in crate::render::frame_graph) struct LitForwardRenderNode {
    /// The index of the descriptor set used for the camera
    camera_descriptor_set: usize,

    /// The index of the descriptor set used for the renderables
    renderable_descriptor_set: usize,

    /// The index of the descriptor set used for the lights
    light_descriptor_set: usize,
}
