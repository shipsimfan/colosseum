mod get;
mod new;

/// A token that represents a specific applied render frame that is ready to be drawn
pub(in crate::render::job) struct RenderToken {
    frame_index: usize,
}
