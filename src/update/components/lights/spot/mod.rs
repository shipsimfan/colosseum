use alexandria::math::{Color3f, Linear, Vector3f};

mod get;
mod new;
mod set;
mod system;

/// A component representing a spot light
pub struct SpotLight {
    /// The color of the spot light
    color: Color3f<Linear>,

    /// The intensity of the light
    intensity: f32,

    /// The position of the light
    position: Vector3f,

    // The range of the light
    range: f32,

    /// The direction of the light
    direction: Vector3f,

    /// The cut-off angle of the light
    cutoff_angle: f32,

    /// The angle at which lights starts falling off
    falloff_angle: f32,
}
