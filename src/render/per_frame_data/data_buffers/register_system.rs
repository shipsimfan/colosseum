use crate::render::{GpuDataBuffers, per_frame_data::data_buffers::CopySystem};
use colosseum_ecs::IntoSystem;

impl GpuDataBuffers {
    /// Registers the system which copies all data from host-local buffers to device-local buffers
    pub fn system(&self) -> impl 'static + IntoSystem<()> {
        CopySystem::new(self)
    }
}
