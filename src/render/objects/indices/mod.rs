use std::marker::PhantomData;

mod get;
mod new;

/// Indices used in the render phase scheduler
pub(crate) struct RenderIndices<Game: crate::Game> {
    /// The index of the window resource
    window: usize,

    /// The index of the settings resource
    settings: usize,

    /// The index of the next scene resource
    next_scene: usize,

    /// The index of the swapchain resource
    swapchain: usize,

    /// The index of the frame index resource
    frame_index: usize,

    /// The index of the frame graph resource
    frame_graph: usize,

    /// The index of the draw fence resource
    draw_fence: usize,

    /// The index of the copy fence resource
    copy_fence: usize,

    /// The index of the GPU data buffers resource
    gpu_data_buffers: usize,

    /// A marker for the game type
    _marker: PhantomData<Game>,
}
