use crate::graphics::{SpotLight, managed_objects::lights::LightList};

mod bind;
mod clear;
mod create;
mod get;
mod index;
mod iter;
mod new;
mod remove;

/// A set of directional lights registered with the engine
pub struct SpotLights {
    /// The list of lights
    list: LightList<SpotLight>,
}
