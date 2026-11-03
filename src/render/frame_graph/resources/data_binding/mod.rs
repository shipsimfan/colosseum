use crate::render::MAX_FRAMES_IN_FLIGHT;

mod new;
mod update;

/// The binding of a descriptor to a data buffer
pub(in crate::render::frame_graph::resources) struct FrameGraphDataBinding {
    /// The epoch for which this data binding is valid
    epoch: [u64; MAX_FRAMES_IN_FLIGHT],

    /// The index of the data buffer this binding corresponds to
    index: usize,

    /// The descriptor sets and bindings which must be updated when the data buffer resizes
    bindings: Vec<(usize, u32)>,
}
