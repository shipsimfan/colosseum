//! The definitions of the different types of lights

mod directional;
mod point;
mod spot;

pub use directional::{DirectionalLight, DirectionalLightHandle};
pub use point::{PointLight, PointLightHandle};
pub use spot::{SpotLight, SpotLightHandle};
