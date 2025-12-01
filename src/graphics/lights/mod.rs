//! The definitions of the different types of lights

mod directional;
mod point;
mod spot;

pub use directional::{DirectionalLight, DirectionalLightInner};
pub use point::{PointLight, PointLightInner};
pub use spot::{SpotLight, SpotLightInner};
