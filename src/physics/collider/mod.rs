use alexandria::math::Vector3f;

mod new;
mod raycast;

/// A collider represents a shape used for physics collision detection
pub enum Collider {
    /// An axis-aligned bounding box collider
    AABB {
        /// The bottom-left-front corner of the axis-aligned bounding box
        min: Vector3f,

        /// The size of the axis-aligned bounding box
        size: Vector3f,
    },
}
