use crate::{
    Result,
    render::{
        CopyCommandBuffer, CopyCompleteSemaphore, CopyFence, FrameIndex, GpuDataBuffers,
        RenderQueue, per_frame_data::data_buffers::CopySystem,
    },
};
use colosseum_ecs::{IntoSystem, System, UnsafeWorldCell, World, WorldAccess};
use std::any::TypeId;

impl System for CopySystem {
    fn initialize(&mut self, world: &mut World) {
        self.copy_command_buffer_index =
            world.get_or_reserve_permanent_resource_index::<CopyCommandBuffer>();
        self.copy_fence_index = world.get_or_reserve_permanent_resource_index::<CopyFence>();
        self.copy_complete_semaphore_index =
            world.get_or_reserve_permanent_resource_index::<CopyCompleteSemaphore>();
        self.render_queue_index = world.get_or_reserve_permanent_resource_index::<RenderQueue>();
        self.frame_index_index = world.get_or_reserve_permanent_resource_index::<FrameIndex>();
        self.gpu_data_buffers_index =
            world.get_or_reserve_permanent_resource_index::<GpuDataBuffers>();
    }

    fn access(&self) -> &WorldAccess {
        &self.access
    }

    fn register_archetype(&mut self, _: usize, _: &[TypeId]) {}

    fn run(&mut self, world: &UnsafeWorldCell) -> Result<()> {
        let mut copy_command_buffer = unsafe {
            world
                .get_mut()
                .get_resource_at_index_mut(self.copy_command_buffer_index)
        };
        let mut copy_fence = unsafe {
            world
                .get_mut()
                .get_resource_at_index_mut(self.copy_fence_index)
        };
        let copy_complete_semaphore = unsafe {
            world
                .get_mut()
                .get_resource_at_index(self.copy_complete_semaphore_index)
        };
        let mut render_queue = unsafe {
            world
                .get_mut()
                .get_resource_at_index_mut(self.render_queue_index)
        };
        let mut gpu_data_buffers = unsafe {
            world
                .get_mut()
                .get_resource_at_index_mut(self.gpu_data_buffers_index)
        };
        let frame_index = unsafe {
            world
                .get_mut()
                .get_resource_at_index(self.frame_index_index)
        };

        CopySystem::run(
            &frame_index,
            &mut copy_command_buffer,
            &copy_complete_semaphore,
            &mut copy_fence,
            &mut render_queue,
            &mut gpu_data_buffers,
            unsafe { world.get_mut() },
        )
    }
}

impl IntoSystem<()> for CopySystem {
    type System = Self;

    fn into_system(self) -> Self::System {
        self
    }
}
