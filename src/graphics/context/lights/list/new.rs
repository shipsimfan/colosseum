use crate::{
    Result,
    graphics::{
        context::{LightType, lights::LightList},
        util::StructuredBuffer,
    },
    util::Arena,
};
use std::num::NonZeroUsize;
use win32::d3d11::ID3D11Device;

impl<T: LightType> LightList<T> {
    /// Create a new [`LightList`]
    pub fn new(capacity: NonZeroUsize, slot: u32, device: &ID3D11Device) -> Result<Self> {
        Ok(LightList {
            arena: Arena::with_capacity(capacity.get()),
            last_len: 0,
            buffer: StructuredBuffer::new(capacity, slot, device)?,
        })
    }
}
