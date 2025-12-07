use crate::{
    math::{Color3f, Vector3f},
    util::Handle,
};

mod get;
mod light_type;
mod new;
mod set;

/// A handle to a [`DirectionalLight`]
pub type DirectionalLightHandle = Handle<DirectionalLight>;

/// A light that is located infinitely far away in a certain direction
pub struct DirectionalLight {
    /// The direction of the light
    direction: Vector3f,

    /// The color of the light
    color: Color3f,

    /// The brightness of the light
    brightness: f32,

    /// Have the values of this light changed this frame?
    dirty: bool,
}
