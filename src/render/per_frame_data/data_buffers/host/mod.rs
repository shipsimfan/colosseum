use crate::render::MAX_FRAMES_IN_FLIGHT;

mod frame;

mod index;
mod new;

pub(in crate::render) use frame::*;

/// A buffer that holds data in host-visible memory for copying to the GPU
pub(in crate::render) struct HostDataBuffer<T> {
    /// The per-frame data for the host data buffer
    frame: [FrameHostDataBuffer<T>; MAX_FRAMES_IN_FLIGHT],
}
