mod execute;

/// The node for the unlit forward pass, which renders all unlit objects in the scene
#[derive(Debug)]
pub(in crate::render::frame_graph) struct UnlitForwardPassNode {
    /// The ID of the output render target
    output: FrameGraphResourceId,
}
