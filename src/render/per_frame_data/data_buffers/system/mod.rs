use colosseum_ecs::WorldAccess;

mod new;
mod run;
mod system;

/// Copies data from host-local buffers to device-local buffers
pub(in crate::render::per_frame_data::data_buffers) struct CopySystem {
    /// The access used by the system
    access: WorldAccess,

    /// The index of the copy command buffer used by the system
    copy_command_buffer_index: usize,

    /// The index of the copy fence used by the system
    copy_fence_index: usize,

    /// The index of the copy complete semaphore used by the system
    copy_complete_semaphore_index: usize,

    /// The index of the render queue used by the system
    render_queue_index: usize,

    /// The index of the gpu data buffers used by the system
    gpu_data_buffers_index: usize,

    /// The index of the frame index used by the system
    frame_index_index: usize,
}
