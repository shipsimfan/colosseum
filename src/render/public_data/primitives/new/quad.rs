use crate::render::Vertex;

/// Create a new quad primitive mesh
pub(in crate::render::public_data::primitives::new) fn quad() -> (Vec<Vertex>, Vec<u32>) {
    (QUAD_VERTICES.to_vec(), QUAD_INDICES.to_vec())
}

/// The vertices for a quad primitive mesh
const QUAD_VERTICES: &[Vertex] = &[
    Vertex::new((-1.0, 1.0, 0.0), (1.0, 1.0, 1.0), (0.0, 0.0, -1.0)),
    Vertex::new((1.0, 1.0, 0.0), (1.0, 1.0, 1.0), (0.0, 0.0, -1.0)),
    Vertex::new((-1.0, -1.0, 0.0), (1.0, 1.0, 1.0), (0.0, 0.0, -1.0)),
    Vertex::new((1.0, -1.0, 0.0), (1.0, 1.0, 1.0), (0.0, 0.0, -1.0)),
];

/// The indices for a quad primitive mesh
const QUAD_INDICES: &[u32] = &[0, 2, 1, 1, 2, 3];
