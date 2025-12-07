use crate::{
    graphics::{managed_objects::lights::LightType, util::StructuredBuffer},
    util::Arena,
};

mod bind;
mod clear;
mod deref;
mod new;

/// A managed list of lights
pub(in crate::graphics::managed_objects::lights) struct LightList<T: LightType> {
    /// The arena containing the lights with stable IDs
    arena: Arena<T>,

    /// The length of the arena last bind
    last_len: usize,

    /// The buffer of the light information on the GPU
    buffer: StructuredBuffer<T::GPU>,
}
