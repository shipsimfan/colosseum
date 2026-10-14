mod deref;
mod new;

/// The current frame index within the rendering pipeline
pub(in crate::render) struct FrameIndex {
    /// The actual index value
    index: usize,
}
