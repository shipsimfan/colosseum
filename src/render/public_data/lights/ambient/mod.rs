use alexandria::math::{Color3f, Linear};

mod default;
mod get;
mod new;
mod set;

/// The ambient light of the scene
pub struct AmbientLight {
    /// The color of the ambient light
    color: Color3f<Linear>,
}
