use alexandria::math::{Matrix4x4f, Vector2u, Vector3f};

mod projection;

mod default;
mod get;
mod new;
mod set;

pub use projection::*;

/// The camera component defining a view point in the scene
///
/// Only one camera can be active at a time
pub struct Camera {
    /// The current projection for the camera
    projection: CameraProjection,

    /// The last seen viewport size, used to determine if the projection matrix needs to be recalculated
    last_viewport_size: Vector2u,

    /// Has the projection matrix been modified since the last time it was calculated?
    projection_dirty: bool,

    /// The projection matrix for the camera
    projection_matrix: Matrix4x4f,

    /// The corners of this camera's view frustum at each cascade depth plane, in view coordinates
    corners: [[Vector3f; 4]; 2],

    /// The depth of the camera's view frustum
    depth: f32,
}
