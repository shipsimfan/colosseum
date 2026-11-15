use alexandria::math::Vector3f;

mod compare;

/// An intersection point from a raycast
pub struct RaycastResult {
    /// The position of the intersection
    pub position: Vector3f,

    /// The distance from the ray origin to the intersection
    pub distance: f32,
}
