use crate::math::{Color3f, Vector3f};

mod get;
mod light_type;
mod new;
mod set;

/// The actual definition of a point light
pub struct PointLightInner {
    /// The position of the light
    position: Vector3f,

    /// The radius of the light
    radius: f32,

    /// The brightness of the light
    brightness: f32,

    /// The color of the light
    color: Color3f,

    /// Have the values of this light changed this frame?
    dirty: bool,
}
