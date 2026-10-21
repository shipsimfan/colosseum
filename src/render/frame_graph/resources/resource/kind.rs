/// The kind of a frame graph resource
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(in crate::render::frame_graph) enum FrameGraphResourceKind {
    /// A resource that matches the native scale of the swapchain
    Native,
}
