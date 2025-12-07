//! The definitions of the different types of lights

use list::LightList;
use r#type::LightType;

mod ambient;
mod directional;
mod list;
mod point;
mod spot;
mod r#type;

mod bind;
mod new;

pub use ambient::AmbientLight;
pub use directional::*;
pub use point::*;
pub use spot::*;

/// The set of registered lights
pub struct Lights {
    /// The ambient light of the scene
    pub ambient: AmbientLight,

    /// The set of directional lights
    pub directional: DirectionalLights,

    /// The set of point lights
    pub point: PointLights,

    /// The set of spot lights
    pub spot: SpotLights,
}
