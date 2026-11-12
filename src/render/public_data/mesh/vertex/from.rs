use crate::render::Vertex;

impl From<parthenon::Vertex> for Vertex {
    fn from(vertex: parthenon::Vertex) -> Self {
        Vertex {
            position: vertex.position,
            color: vertex.color.into_f32(),
            normal: vertex.normal,
        }
    }
}
