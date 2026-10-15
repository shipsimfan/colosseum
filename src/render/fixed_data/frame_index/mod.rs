mod deref;
mod increment;
mod new;

/// The current frame index within the rendering pipeline
#[derive(Debug, Clone)]
pub(in crate::render) struct FrameIndex {
    /// The actual index value
    index: usize,
}
