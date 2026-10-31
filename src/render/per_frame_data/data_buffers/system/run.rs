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
        copy_complete_semaphore: &mut CopyCompleteSemaphore,
        copy_fence: &mut CopyFence,
        render_queue: &mut RenderQueue,
        gpu_data_buffers: &mut GpuDataBuffers,
        world: &mut World,
    ) -> Result<()> {
        let cmd_buffer = &mut copy_command_buffer[frame_index];
        cmd_buffer
            .begin(None)
            .map_err(|error| new_error!("unable to begin copy command buffer - {}", error))?;

        #[cfg(debug_assertions)]
        let mut debug_label =
            cmd_buffer.cmd_begin_debug_utils_label(c"Copy", (0.85, 0.75, 0.3, 1.0));

        for data_buffer in &mut gpu_data_buffers.buffers {
            data_buffer.copy(
                &mut debug_label,
                frame_index,
                &gpu_data_buffers.device,
                world,
            )?;
        }

        drop(debug_label);

        cmd_buffer
            .end()
            .map_err(|error| new_error!("unable to end copy command buffer - {}", error))?;

        let (copy_complete_semaphore, timeline_value) = &mut copy_complete_semaphore[frame_index];
        *timeline_value += 1;
        render_queue
            .submit(
                &[VulkanSubmitInfo::new(
                    0,
                    &[],
                    &[VulkanCommandBufferSubmitInfo::new(cmd_buffer, 0)],
                    &[VulkanSemaphoreSubmitInfo::new(
                        &copy_complete_semaphore,
                        *timeline_value,
                        VulkanPipelineStageFlag::Copy,
                        0,
                        None,
                    )],
                )],
                Some(copy_fence.get_mut(frame_index)),
            )
            .map_err(|error| new_error!("unable to submit copy commands - {}", error))?;

        Ok(())
    }
}
