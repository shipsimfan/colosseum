use crate::{
    FileIo,
    render::{AssetLoader, GpuTransferQueue, GraphicsDevice},
};
use std::sync::Arc;

impl AssetLoader {
    /// Create a new [`AssetLoader`]
    pub(in crate::render::public_data) fn new(
        device: &Arc<GraphicsDevice>,
        gpu_transfer_queue: GpuTransferQueue,
        file_io: FileIo,
    ) -> AssetLoader {
        AssetLoader {
            device: device.clone(),
            gpu_transfer_queue,
            file_io,
        }
    }
}
