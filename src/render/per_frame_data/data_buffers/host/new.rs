use crate::{
    Result,
    render::{FrameHostDataBuffer, GraphicsDevice, HostDataBuffer, MAX_FRAMES_IN_FLIGHT},
};

impl<T> HostDataBuffer<T> {
    /// Create a new [`HostDataBuffer`]
    pub(in crate::render::per_frame_data::data_buffers) fn new(
        name: &str,
        capacity: usize,
        device: &GraphicsDevice,
    ) -> Result<HostDataBuffer<T>> {
        Ok(HostDataBuffer {
            frame: [
                FrameHostDataBuffer::new(name, 0, capacity, device)?,
                FrameHostDataBuffer::new(name, 1, capacity, device)?,
                FrameHostDataBuffer::new(name, 2, capacity, device)?,
            ],
        })
    }
}

impl<T: Clone> HostDataBuffer<T> {
    /// Create a new [`HostDataBuffer`] with some initial data
    pub(in crate::render::per_frame_data::data_buffers) fn with_data(
        name: &str,
        initial_data: &[T],
        device: &GraphicsDevice,
    ) -> Result<HostDataBuffer<T>> {
        let mut host_data_buffer = HostDataBuffer::new(name, initial_data.len(), device)?;

        for item in initial_data {
            for i in 0..MAX_FRAMES_IN_FLIGHT {
                host_data_buffer.frame[i].push(item.clone());
            }
        }

        Ok(host_data_buffer)
    }
}
