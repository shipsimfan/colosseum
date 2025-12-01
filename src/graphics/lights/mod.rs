//! The definitions of the different types of lights

mod directional;
mod point;

pub use directional::{DirectionalLight, DirectionalLightInner};
pub use point::{PointLight, PointLightInner};
