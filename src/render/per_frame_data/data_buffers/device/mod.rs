use crate::render::MAX_FRAMES_IN_FLIGHT;

mod frame;
mod r#trait;

mod new;

pub(in crate::render::per_frame_data::data_buffers) use frame::*;
pub(in crate::render::per_frame_data::data_buffers) use r#trait::*;

/// A buffer that holds data in device-local memory
pub(in crate::render::per_frame_data::data_buffers) struct DeviceDataBuffer<T> {
    /// The index of the corresponding host data buffer resource
    host_index: usize,

    /// The per-frame data for the device data buffer
    frame: [FrameDeviceDataBuffer<T>; MAX_FRAMES_IN_FLIGHT],
}
