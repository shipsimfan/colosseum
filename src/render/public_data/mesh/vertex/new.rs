use crate::render::Vertex;
use alexandria::math::{Color3f, Linear, Vector3f};

impl Vertex {
    /// Create a new [`Vertex`]
    pub const fn new<
        Position: [const] Into<Vector3f>,
        Color: [const] Into<Color3f<Linear>>,
        Normal: [const] Into<Vector3f>,
    >(
        position: Position,
        color: Color,
        normal: Normal,
    ) -> Vertex {
        Vertex {
            position: position.into(),
            color: color.into(),
            normal: normal.into(),
        }
    }
}
