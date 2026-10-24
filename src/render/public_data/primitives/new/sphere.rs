use crate::render::Vertex;
use alexandria::math::Vector3f;
use std::collections::BTreeMap;

/// Create a new icosphere primitive mesh
pub(in crate::render::public_data::primitives::new) fn sphere() -> (Vec<Vertex>, Vec<u32>) {
    const PHI: f32 = 1.61803398875;
    const SUBDIVISIONS: usize = 4;

    let p0 = Vector3f::new(-1.0, PHI, 0.0).normalized();
    let p1 = Vector3f::new(1.0, PHI, 0.0).normalized();
    let p2 = Vector3f::new(-1.0, -PHI, 0.0).normalized();
    let p3 = Vector3f::new(1.0, -PHI, 0.0).normalized();
    let p4 = Vector3f::new(0.0, -1.0, PHI).normalized();
    let p5 = Vector3f::new(0.0, 1.0, PHI).normalized();
    let p6 = Vector3f::new(0.0, -1.0, -PHI).normalized();
    let p7 = Vector3f::new(0.0, 1.0, -PHI).normalized();
    let p8 = Vector3f::new(PHI, 0.0, -1.0).normalized();
    let p9 = Vector3f::new(PHI, 0.0, 1.0).normalized();
    let p10 = Vector3f::new(-PHI, 0.0, -1.0).normalized();
    let p11 = Vector3f::new(-PHI, 0.0, 1.0).normalized();

    let mut vertices = vec![
        Vertex::new(p0, (1.0, 1.0, 1.0), p0),
        Vertex::new(p1, (1.0, 1.0, 1.0), p1),
        Vertex::new(p2, (1.0, 1.0, 1.0), p2),
        Vertex::new(p3, (1.0, 1.0, 1.0), p3),
        Vertex::new(p4, (1.0, 1.0, 1.0), p4),
        Vertex::new(p5, (1.0, 1.0, 1.0), p5),
        Vertex::new(p6, (1.0, 1.0, 1.0), p6),
        Vertex::new(p7, (1.0, 1.0, 1.0), p7),
        Vertex::new(p8, (1.0, 1.0, 1.0), p8),
        Vertex::new(p9, (1.0, 1.0, 1.0), p9),
        Vertex::new(p10, (1.0, 1.0, 1.0), p10),
        Vertex::new(p11, (1.0, 1.0, 1.0), p11),
    ];

    let mut indices = vec![
        0, 5, 11, 0, 1, 5, 0, 7, 1, 0, 10, 7, 0, 11, 10, 1, 9, 5, 5, 4, 11, 11, 2, 10, 10, 6, 7, 7,
        8, 1, 3, 4, 9, 3, 2, 4, 3, 6, 2, 3, 8, 6, 3, 9, 8, 4, 5, 9, 2, 11, 4, 6, 10, 2, 8, 7, 6, 9,
        1, 8,
    ];

    let mut midpoint_cache = BTreeMap::new();

    for _ in 0..SUBDIVISIONS {
        let mut new_indices = Vec::with_capacity(indices.len() * 4);

        for chunk in indices.chunks(3) {
            let (a, b, c) = (chunk[0], chunk[1], chunk[2]);

            let ab = midpoint(a, b, &mut vertices, &mut midpoint_cache);
            let bc = midpoint(b, c, &mut vertices, &mut midpoint_cache);
            let ca = midpoint(c, a, &mut vertices, &mut midpoint_cache);

            new_indices.extend_from_slice(&[a, ca, ab, b, ab, bc, c, bc, ca, ab, ca, bc]);
        }

        indices = new_indices;
    }

    (vertices, indices)
}

/// Calculate the midpoint vertex between two vertices, caching the result to avoid duplicates
fn midpoint(
    a: u32,
    b: u32,
    vertices: &mut Vec<Vertex>,
    midpoint_cache: &mut BTreeMap<(u32, u32), u32>,
) -> u32 {
    let key = (a.min(b), a.max(b));
    if let Some(&midpoint_index) = midpoint_cache.get(&key) {
        return midpoint_index;
    }

    let v1 = vertices[a as usize].position();
    let v2 = vertices[b as usize].position();
    let mid = ((v1 + v2) / 2.0).normalized();

    let index = vertices.len() as u32;
    vertices.push(Vertex::new(mid, (1.0, 1.0, 1.0), mid));
    midpoint_cache.insert(key, index);
    index
}
