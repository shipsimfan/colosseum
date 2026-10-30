use alexandria::gpu::{
    VulkanCommandBufferSubmitInfo, VulkanPipelineStageFlag, VulkanSemaphoreSubmitInfo,
    VulkanSubmitInfo,
};
use colosseum_core::new_error;
use colosseum_ecs::World;

use crate::{
    Result,
    render::{
        CopyCommandBuffer, CopyCompleteSemaphore, CopyFence, FrameIndex, GpuDataBuffers,
        RenderQueue, per_frame_data::data_buffers::CopySystem,
    },
};

impl CopySystem {
    /// Run the copy system
    pub(in crate::render::per_frame_data::data_buffers::system) fn run(
        frame_index: &FrameIndex,
        copy_command_buffer: &mut CopyCommandBuffer,
        copy_complete_semaphore: &CopyCompleteSemaphore,
        copy_fence: &mut CopyFence,
        render_queue: &mut RenderQueue,
        gpu_data_buffers: &mut GpuDataBuffers,
        world: &mut World,
    ) -> Result<()> {
        let cmd_buffer = &mut copy_command_buffer[frame_index];
        cmd_buffer
            .begin(None)
            .map_err(|error| new_error!("unable to begin copy command buffer - {}", error))?;

        for data_buffer in &mut gpu_data_buffers.buffers {
            data_buffer.copy(cmd_buffer, frame_index, &gpu_data_buffers.device, world)?;
        }

        cmd_buffer
            .end()
            .map_err(|error| new_error!("unable to end copy command buffer - {}", error))?;

        render_queue
            .submit(
                &[VulkanSubmitInfo::new(
                    0,
                    &[],
                    &[VulkanCommandBufferSubmitInfo::new(cmd_buffer, 0)],
                    &[VulkanSemaphoreSubmitInfo::new(
                        &copy_complete_semaphore[frame_index],
                        0,
                        VulkanPipelineStageFlag::Copy,
                        0,
                    )],
                )],
                Some(copy_fence.get_mut(frame_index)),
            )
            .map_err(|error| new_error!("unable to submit copy commands - {}", error))
    }
}
