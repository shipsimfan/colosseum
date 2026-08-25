/// How a resource is written to by a node
pub(in crate::render::frame_graph) enum FrameGraphResourceUsage {
    /// The resource is written to as a color attachment
    ColorAttachment,

    /// The resource is written to as a depth attachment
    DepthAttachment,

    /// The resource is written to as a transfer destination
    TransferDst,

    /// The resource is read from as a transfer source
    TransferSrc,
}
