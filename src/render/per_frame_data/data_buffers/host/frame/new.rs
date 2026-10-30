use crate::{
    Result,
    render::{FrameHostDataBuffer, GraphicsDevice},
};

impl<T> FrameHostDataBuffer<T> {
    /// Create a new [`FrameHostDataBuffer`]
    pub(in crate::render::per_frame_data::data_buffers::host) fn new(
        name: &str,
        index: usize,
        capacity: usize,
        device: &GraphicsDevice,
    ) -> Result<FrameHostDataBuffer<T>> {
        let buffer_name = format!("Host {} {}", name, index);
        let memory_name = format!("Host {} Memory {}", name, index);

        let (buffer, memory, stride) =
            FrameHostDataBuffer::allocate_memory(&buffer_name, &memory_name, capacity, device)?;

        Ok(FrameHostDataBuffer {
            buffer_name,
            memory_name,
            capacity,
            count: 0,
            buffer,
            memory,
            stride,
        })
    }
}
