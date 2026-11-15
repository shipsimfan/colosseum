use alexandria::math::Vector3f;

mod result;

mod from_screen_position;
mod new;
mod set;

pub use result::*;

/// A ray represents a line segment used for physics raycasting
pub struct Ray {
    /// The origin of the ray
    pub origin: Vector3f,

    /// The direction of the ray
    pub direction: Vector3f,
}
