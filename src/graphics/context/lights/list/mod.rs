use crate::{graphics::util::StructuredBuffer, util::Arena};

mod bind;
mod deref;
mod light_type;
mod new;

pub(in crate::graphics) use light_type::LightType;

/// A managed list of lights
pub(in crate::graphics::context::lights) struct LightList<T: LightType> {
    /// The arena containing the lights with stable IDs
    arena: Arena<T>,

    /// The length of the arena last bind
    last_len: usize,

    /// The buffer of the light information on the GPU
    buffer: StructuredBuffer<T::GPU>,
}
