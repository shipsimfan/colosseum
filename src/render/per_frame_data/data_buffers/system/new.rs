use crate::render::{
    CopyCommandBuffer, CopyCompleteSemaphore, CopyFence, FrameIndex, GpuDataBuffers, RenderQueue,
    per_frame_data::data_buffers::CopySystem,
};
use colosseum_ecs::WorldAccess;

impl CopySystem {
    /// Creates a new [`CopySystem`]
    pub fn new(data_buffers: &GpuDataBuffers) -> CopySystem {
        let mut access = WorldAccess::None;
        access.write_resource::<CopyCommandBuffer>();
        access.read_resource::<CopyCompleteSemaphore>();
        access.write_resource::<CopyFence>();
        access.write_resource::<RenderQueue>();
        access.write_resource::<GpuDataBuffers>();
        access.read_resource::<FrameIndex>();

        for buffer in &data_buffers.buffers {
            buffer.access(&mut access);
        }

        CopySystem {
            access,
            copy_command_buffer_index: 0,
            copy_fence_index: 0,
            copy_complete_semaphore_index: 0,
            render_queue_index: 0,
            gpu_data_buffers_index: 0,
            frame_index_index: 0,
        }
    }
}
