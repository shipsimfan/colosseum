use crate::render::ObjectData;
use alexandria::math::Matrix4x4f;

impl ObjectData {
    /// Create a new [`ObjectData`]
    pub fn new(model: Matrix4x4f) -> ObjectData {
        ObjectData { model }
    }
}
