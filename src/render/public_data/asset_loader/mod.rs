use crate::{
    FileIo,
    render::{GpuTransferQueue, GraphicsDevice},
};
use std::sync::Arc;

mod create_shader;
mod new;

/// Loads game assets
pub struct AssetLoader {
    /// The device for creating objects and allocating memory
    device: Arc<GraphicsDevice>,

    /// The queue for transfering GPU assets
    gpu_transfer_queue: GpuTransferQueue,

    /// The thread for reading files
    file_io: FileIo,
}
