use commands::GpuTransferCommand;
use staging_buffer::StagingBuffer;
use std::sync::mpsc::Sender;

mod commands;
mod mesh;
mod render;
mod staging_buffer;

mod new;
mod thread;
mod transfer;

pub use mesh::*;

pub(in crate::render) use render::*;

/// The queue for transferring data to the GPU
pub(crate) struct GpuTransferQueue {
    /// The sender for the transfer commands
    queue: Sender<GpuTransferCommand>,
}
