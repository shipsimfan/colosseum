use crate::math::{Color3f, Vector3f};

mod get;
mod light_type;
mod new;
mod set;

/// The actual definition of a point light
pub struct SpotLightInner {
    /// The position of the light
    position: Vector3f,

    /// The distance the light shines
    distance: f32,

    /// The direction of the light
    direction: Vector3f,

    /// The cut-off angle of the light
    cut_off: f32,

    /// The color of the light
    color: Color3f,

    /// The brightness of the light
    brightness: f32,

    /// Have the values of this light changed this frame?
    dirty: bool,
}
