use crate::{
    FileIo,
    render::{Assets, GpuTransferQueue, GraphicsDevice, public_data::assets::GpuAllocator},
};
use alexandria::SlotMap;
use std::sync::Arc;

impl Assets {
    /// Create a new [`Assets`]
    pub(in crate::render::public_data) fn new(
        device: &Arc<GraphicsDevice>,
        gpu_transfer_queue: GpuTransferQueue,
        file_io: FileIo,
    ) -> Assets {
        Assets {
            device: device.clone(),
            gpu_transfer_queue,
            file_io,
            mesh_allocator: GpuAllocator::new(
                "Mesh Allocator",
                16 * 1024 * 1024, // 16 MB
                256,              // 256 B
                4 * 1024 * 1024,  // 4 MB
                device,
            ),
            meshes: SlotMap::new(),
        }
    }
}
