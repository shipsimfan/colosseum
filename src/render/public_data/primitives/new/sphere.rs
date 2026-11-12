use crate::render::Vertex;
use alexandria::math::Vector3f;
use std::collections::BTreeMap;

/// Create a new icosphere primitive mesh
pub(in crate::render::public_data::primitives::new) fn sphere() -> (Vec<Vertex>, Vec<[u32; 3]>) {
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
        [0, 11, 5],
        [0, 5, 1],
        [0, 1, 7],
        [0, 7, 10],
        [0, 10, 11],
        [1, 5, 9],
        [5, 11, 4],
        [11, 10, 2],
        [10, 7, 6],
        [7, 1, 8],
        [3, 9, 4],
        [3, 4, 2],
        [3, 2, 6],
        [3, 6, 8],
        [3, 8, 9],
        [4, 9, 5],
        [2, 4, 11],
        [6, 2, 10],
        [8, 6, 7],
        [9, 8, 1],
    ];

    let mut midpoint_cache = BTreeMap::new();

    for _ in 0..SUBDIVISIONS {
        let mut new_indices = Vec::with_capacity(indices.len() * 4);

        for [a, b, c] in indices {
            let ab = midpoint(a, b, &mut vertices, &mut midpoint_cache);
            let bc = midpoint(b, c, &mut vertices, &mut midpoint_cache);
            let ca = midpoint(c, a, &mut vertices, &mut midpoint_cache);

            new_indices.push([a, ab, ca]);
            new_indices.push([b, bc, ab]);
            new_indices.push([c, ca, bc]);
            new_indices.push([ab, bc, ca]);
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
