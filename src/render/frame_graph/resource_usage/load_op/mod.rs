use alexandria::math::{Color4f, Linear};

mod to_vk;

/// A load operation for a resource in the frame graph, which specifies how the resource should be
/// loaded at the start of a render pass
pub(in crate::render::frame_graph) enum FrameGraphResourceLoadOp {
    /// The resource is cleared to a specific value at the start of the render pass
    Clear {
        /// The color to clear the resource to
        ///
        /// TODO: Remove this when we support depth/stencil attachments
        color: Color4f<Linear>,
    },

    /// The resource is loaded from its previous contents at the start of the render pass
    Load,

    /// The value loaded does not matter for this render pass
    DontCare,
}
