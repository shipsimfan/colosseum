use crate::graphics::{PointLight, managed_objects::lights::LightList};

mod bind;
mod clear;
mod create;
mod get;
mod index;
mod iter;
mod new;
mod remove;

/// A set of directional lights registered with the engine
pub struct PointLights {
    /// The list of lights
    list: LightList<PointLight>,
}
