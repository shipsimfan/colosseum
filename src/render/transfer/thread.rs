use crate::{
    Result, UserEvent, debug,
    render::{GpuTransferQueue, RenderGpuTransferQueue},
};
use alexandria::gpu::VulkanQueue;
use colosseum_core::{GlobalSharedState, Logger};

impl GpuTransferQueue {
    pub(in crate::render::transfer) fn thread(
        shared_state: &GlobalSharedState<UserEvent>,
        mut queue: VulkanQueue,
        mut transfer_queue: RenderGpuTransferQueue,
        logger: Logger,
    ) -> Result<()> {
        while shared_state.is_running() {
            if !transfer_queue.handle_command(&mut queue, true)? {
                debug!(&logger, "Transfer thread exiting due to channel closure");
                break;
            }
        }

        Ok(())
    }
}
