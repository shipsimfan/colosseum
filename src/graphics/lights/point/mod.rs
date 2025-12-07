use crate::{
    math::{Color3f, Vector3f},
    util::Handle,
};

mod get;
mod light_type;
mod new;
mod set;

/// A handle to a [`PointLight`]
pub type PointLightHandle = Handle<PointLight>;

/// A light that is located at a specific point and shines outwards equally in all directions
pub struct PointLight {
    /// The position of the light
    position: Vector3f,

    /// The radius of the light
    radius: f32,

    /// The color of the light
    color: Color3f,

    /// The brightness of the light
    brightness: f32,

    /// Have the values of this light changed this frame?
    dirty: bool,
}
