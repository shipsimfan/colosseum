mod get;
mod new;

/// An ID identifying a resource in the frame graph
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub(in crate::render::frame_graph) struct FrameGraphResourceId {
    /// The ID of the resource
    ///
    /// The top bit of the ID is used to indicate if this is a transient or external resource. If
    /// the top bit is set, this is an external resource. Otherwise, this is a transient resource.
    id: usize,
}

impl FrameGraphResourceId {
    /// The number of bits used to store the type of the resource ID
    const TYPE_BITS: u32 = 2;

    /// The shift amount to get the type of the resource ID
    const TYPE_SHIFT: u32 = usize::BITS - FrameGraphResourceId::TYPE_BITS;

    /// The mask used to get the type of the resource ID
    const TYPE_MASK: usize =
        ((1 << FrameGraphResourceId::TYPE_BITS) - 1) << FrameGraphResourceId::TYPE_SHIFT;

    /// The bit used to indicate that a resource ID is transient and at render scale
    const TRANSIENT_RENDER_SCALE: usize = 0b00 << FrameGraphResourceId::TYPE_SHIFT;

    /// The bit used to indicate that a resource ID is transient and at native scale
    const TRANSIENT_NATIVE_SCALE: usize = 0b01 << FrameGraphResourceId::TYPE_SHIFT;

    /// The bit used to indicate that a resource ID is external (e.g., from the swapchain)
    const EXTERNAL: usize = 0b11 << FrameGraphResourceId::TYPE_SHIFT;

    /// The ID for the swapchain image, which is always an external resource with ID 0
    pub const SWAPCHAIN_IMAGE: FrameGraphResourceId = FrameGraphResourceId::new_external(0);
}
