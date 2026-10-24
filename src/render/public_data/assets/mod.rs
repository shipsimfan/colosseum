use crate::{
    FileIo,
    render::{GpuTransferQueue, GraphicsDevice, Mesh},
};
use alexandria::SlotMap;
use std::sync::Arc;

mod allocator;

mod complete_mesh;
mod create_mesh;
mod create_shader;
mod get;
mod new;
mod remove_mesh;

pub(in crate::render) use allocator::*;

/// The assets that have been loaded into the game
pub struct Assets {
    /// The device for creating objects and allocating memory
    device: Arc<GraphicsDevice>,

    /// The queue for transfering GPU assets
    gpu_transfer_queue: GpuTransferQueue,

    /// The thread for reading files
    file_io: FileIo,

    /// The allocator for meshes
    mesh_allocator: GpuAllocator,

    /// The set of meshes that have been loaded
    meshes: SlotMap<Mesh>,
}
