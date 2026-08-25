/// How a resource is written to by a node
pub(in crate::render::frame_graph) enum FrameGraphResourceUsage {
    /// The resource is written to as a color attachment
    ColorAttachment,

    /// The resource is written to as a depth attachment
    DepthAttachment,
}
