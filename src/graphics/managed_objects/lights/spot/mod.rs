use crate::{
    math::{Color3f, Vector3f},
    util::Handle,
};

mod spot_lights;

mod get;
mod light_type;
mod new;
mod set;

pub use spot_lights::SpotLights;

/// A handle to a [`SpotLight`]
pub type SpotLightHandle = Handle<SpotLight>;

/// A light that is located at a specific point and shines outwards in a specific direction
pub struct SpotLight {
    /// The position of the light
    position: Vector3f,

    /// The distance the light shines
    distance: f32,

    /// The direction of the light
    direction: Vector3f,

    /// The angle to which the light shines with full intensity
    inner_angle: f32,

    /// The angle at which the light stops illuminating completely
    outer_angle: f32,

    /// The color of the light
    color: Color3f,

    /// The brightness of the light
    brightness: f32,

    /// Have the values of this light changed this frame?
    dirty: bool,
}
