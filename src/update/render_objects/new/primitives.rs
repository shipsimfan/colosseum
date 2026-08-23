use std::collections::BTreeMap;

use alexandria::math::Vector3f;

use crate::render::Vertex;

pub(in crate::update::render_objects::new) const QUAD_VERTICES: &[Vertex] = &[
    Vertex::new((-1.0, 1.0, 0.0), (1.0, 1.0, 1.0)),
    Vertex::new((1.0, 1.0, 0.0), (1.0, 1.0, 1.0)),
    Vertex::new((-1.0, -1.0, 0.0), (1.0, 1.0, 1.0)),
    Vertex::new((1.0, -1.0, 0.0), (1.0, 1.0, 1.0)),
];
pub(in crate::update::render_objects::new) const QUAD_INDICES: &[u32] = &[0, 2, 1, 1, 2, 3];

pub(in crate::update::render_objects::new) const CUBE_VERTICES: &[Vertex] = &[
    Vertex::new((-1.0, 1.0, 1.0), (1.0, 1.0, 1.0)),
    Vertex::new((1.0, 1.0, 1.0), (1.0, 1.0, 1.0)),
    Vertex::new((-1.0, -1.0, 1.0), (1.0, 1.0, 1.0)),
    Vertex::new((1.0, -1.0, 1.0), (1.0, 1.0, 1.0)),
    Vertex::new((-1.0, 1.0, -1.0), (1.0, 1.0, 1.0)),
    Vertex::new((1.0, 1.0, -1.0), (1.0, 1.0, 1.0)),
    Vertex::new((-1.0, -1.0, -1.0), (1.0, 1.0, 1.0)),
    Vertex::new((1.0, -1.0, -1.0), (1.0, 1.0, 1.0)),
];
pub(in crate::update::render_objects::new) const CUBE_INDICES: &[u32] = &[
    0, 1, 2, 1, 3, 2, 4, 6, 5, 5, 6, 7, 0, 4, 1, 1, 4, 5, 2, 3, 6, 3, 7, 6, 0, 2, 4, 2, 6, 4, 1, 5,
    3, 3, 5, 7,
];

/// Generate a plane primitive mesh
pub(in crate::update::render_objects::new) fn plane() -> (Vec<Vertex>, Vec<u32>) {
    const WIDTH: usize = 10;
    const HEIGHT: usize = 10;

    const VERTEX_COUNT: usize = (WIDTH + 1) * (HEIGHT + 1);

    let mut vertices = Vec::with_capacity(VERTEX_COUNT);
    let mut indices = Vec::with_capacity(WIDTH * HEIGHT * 6);

    let color = (1.0, 1.0, 1.0);
    for y in 0..=HEIGHT {
        for x in 0..=WIDTH {
            let position = (
                x as f32 - WIDTH as f32 / 2.0,
                y as f32 - HEIGHT as f32 / 2.0,
                0.0,
            );
            vertices.push(Vertex::new(position, color));
        }
    }

    for y in 0..HEIGHT {
        for x in 0..WIDTH {
            let top_left = ((y + 1) * (WIDTH + 1) + x) as u32;
            let top_right = top_left + 1;
            let bottom_left = (y * (WIDTH + 1) + x) as u32;
            let bottom_right = bottom_left + 1;

            indices.push(top_left);
            indices.push(bottom_left);
            indices.push(top_right);

            indices.push(top_right);
            indices.push(bottom_left);
            indices.push(bottom_right);
        }
    }

    (vertices, indices)
}

/// Generate an icosphere primitive mesh
pub(in crate::update::render_objects::new) fn sphere() -> (Vec<Vertex>, Vec<u32>) {
    const PHI: f32 = 1.61803398875;
    const SUBDIVISIONS: usize = 4;

    let mut vertices = vec![
        Vertex::new(Vector3f::new(-1.0, PHI, 0.0).normalized(), (1.0, 1.0, 1.0)),
        Vertex::new(Vector3f::new(1.0, PHI, 0.0).normalized(), (1.0, 1.0, 1.0)),
        Vertex::new(Vector3f::new(-1.0, -PHI, 0.0).normalized(), (1.0, 1.0, 1.0)),
        Vertex::new(Vector3f::new(1.0, -PHI, 0.0).normalized(), (1.0, 1.0, 1.0)),
        Vertex::new(Vector3f::new(0.0, -1.0, PHI).normalized(), (1.0, 1.0, 1.0)),
        Vertex::new(Vector3f::new(0.0, 1.0, PHI).normalized(), (1.0, 1.0, 1.0)),
        Vertex::new(Vector3f::new(0.0, -1.0, -PHI).normalized(), (1.0, 1.0, 1.0)),
        Vertex::new(Vector3f::new(0.0, 1.0, -PHI).normalized(), (1.0, 1.0, 1.0)),
        Vertex::new(Vector3f::new(PHI, 0.0, -1.0).normalized(), (1.0, 1.0, 1.0)),
        Vertex::new(Vector3f::new(PHI, 0.0, 1.0).normalized(), (1.0, 1.0, 1.0)),
        Vertex::new(Vector3f::new(-PHI, 0.0, -1.0).normalized(), (1.0, 1.0, 1.0)),
        Vertex::new(Vector3f::new(-PHI, 0.0, 1.0).normalized(), (1.0, 1.0, 1.0)),
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
    vertices.push(Vertex::new(mid, (1.0, 1.0, 1.0)));
    midpoint_cache.insert(key, index);
    index
}

/// Generate a cylinder primitive mesh
pub(in crate::update::render_objects::new) fn cylinder() -> (Vec<Vertex>, Vec<u32>) {
    const SEGMENTS: usize = 32;
    const HEIGHT: f32 = 2.0;

    let mut vertices = Vec::with_capacity(SEGMENTS * 2 + 2);
    let mut indices = Vec::with_capacity(SEGMENTS * 12);

    let color = (1.0, 1.0, 1.0);
    for i in 0..SEGMENTS {
        let angle = (i as f32 / SEGMENTS as f32) * std::f32::consts::TAU;
        let x = angle.cos();
        let z = angle.sin();
        vertices.push(Vertex::new((x, HEIGHT / 2.0, z), color));
        vertices.push(Vertex::new((x, -HEIGHT / 2.0, z), color));
    }

    // Top and bottom center vertices
    vertices.push(Vertex::new((0.0, HEIGHT / 2.0, 0.0), color));
    vertices.push(Vertex::new((0.0, -HEIGHT / 2.0, 0.0), color));

    for i in 0..SEGMENTS {
        let next = (i + 1) % SEGMENTS;

        // Side faces
        indices.push(i as u32 * 2);
        indices.push(i as u32 * 2 + 1);
        indices.push(next as u32 * 2);

        indices.push(next as u32 * 2);
        indices.push(i as u32 * 2 + 1);
        indices.push(next as u32 * 2 + 1);

        // Top face
        indices.push(SEGMENTS as u32 * 2);
        indices.push(i as u32 * 2);
        indices.push(next as u32 * 2);

        // Bottom face
        indices.push(SEGMENTS as u32 * 2 + 1);
        indices.push(next as u32 * 2 + 1);
        indices.push(i as u32 * 2 + 1);
    }

    (vertices, indices)
}
