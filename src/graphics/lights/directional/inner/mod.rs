use crate::math::{Color3f, Vector3f};

mod get;
mod light_type;
mod new;
mod set;

/// The actual definition of a directional light
pub struct DirectionalLightInner {
    /// The direction of the light
    direction: Vector3f,

    /// The color of the light
    color: Color3f,

    /// The brightness of the light
    brightness: f32,

    /// Have the values of this light changed this frame?
    dirty: bool,
}
