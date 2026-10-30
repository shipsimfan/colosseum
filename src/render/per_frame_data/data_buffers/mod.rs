use crate::render::GraphicsDevice;
use device::*;
use std::sync::Arc;
use system::*;

mod system;

mod create;
mod device;
mod host;
mod register_system;

mod new;

pub(in crate::render) use host::*;

/// The manager for device data buffers
pub(in crate::render) struct GpuDataBuffers {
    /// The created data buffers
    buffers: Vec<Box<dyn TypelessDeviceDataBuffer>>,

    /// The graphics device for creating buffers
    device: Arc<GraphicsDevice>,
}
