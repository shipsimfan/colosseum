use alexandria::math::Matrix4x4f;

mod new;

/// The object passed to a shader for a single object
pub(crate) struct ObjectData {
    /// The model matrix of the object
    model: Matrix4x4f,
}
