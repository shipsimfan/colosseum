use crate::graphics::{DirectionalLight, managed_objects::lights::LightList};

mod bind;
mod create;
mod get;
mod index;
mod iter;
mod new;
mod remove;

/// A set of directional lights registered with the engine
pub struct DirectionalLights {
    /// The list of lights
    list: LightList<DirectionalLight>,
}
