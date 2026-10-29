use crate::render::MAX_FRAMES_IN_FLIGHT;
use device::*;
use host::*;

mod device;
mod host;

mod new;

/// A contiguous buffer that holds a set number of elements in both a device-local and host-visible
/// memory block
pub(in crate::render) struct GpuDataBuffer<T> {
    /// The name of the data buffer
    name: String,

    /// The per-frame data for the GPU data buffer
    inner: [GpuDataBufferInner<T>; MAX_FRAMES_IN_FLIGHT],
}

/// The per-frame data for a given GPU data buffer
struct GpuDataBufferInner<T> {
    /// The capacity of the buffer
    capacity: usize,

    /// The current number of objects in the buffer
    count: usize,

    /// The host-visible buffer for writing object data
    host_buffer: HostDataBuffer<T>,

    /// The device-local buffer for GPU access
    device_buffer: DeviceDataBuffer<T>,
}
