use alexandria::math::{Color3f, Linear, Vector3f};

mod get;
mod new;
mod set;
mod system;

/// A component representing a directional light
pub struct DirectionalLight {
    /// The color of the directional light
    color: Color3f<Linear>,

    /// The intensity of the light
    intensity: f32,

    /// The direction of the light
    direction: Vector3f,
}
