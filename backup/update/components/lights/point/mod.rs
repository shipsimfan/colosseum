use alexandria::math::{Color3f, Linear, Matrix4x4f, Vector3f};

mod get;
mod new;
mod set;
mod system;

/// A component representing a point light
pub struct PointLight {
    /// The color of the point light
    color: Color3f<Linear>,

    /// The intensity of the light
    intensity: f32,

    /// The position of the light
    position: Vector3f,

    /// The range of the light
    range: f32,

    /// The view-projection matrices for this light
    view_projections: [Matrix4x4f; 6],

    /// Do the view-projection matrices need to be re-calcualated?
    dirty: bool,
}
