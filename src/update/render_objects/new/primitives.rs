use crate::render::Vertex;
use alexandria::math::Vector3f;
use std::collections::BTreeMap;

pub(in crate::update::render_objects::new) const QUAD_VERTICES: &[Vertex] = &[
    Vertex::new((-1.0, 1.0, 0.0), (1.0, 1.0, 1.0), (0.0, 0.0, -1.0)),
    Vertex::new((1.0, 1.0, 0.0), (1.0, 1.0, 1.0), (0.0, 0.0, -1.0)),
    Vertex::new((-1.0, -1.0, 0.0), (1.0, 1.0, 1.0), (0.0, 0.0, -1.0)),
    Vertex::new((1.0, -1.0, 0.0), (1.0, 1.0, 1.0), (0.0, 0.0, -1.0)),
];
pub(in crate::update::render_objects::new) const QUAD_INDICES: &[u32] = &[0, 2, 1, 1, 2, 3];

pub(in crate::update::render_objects::new) const CUBE_VERTICES: &[Vertex] = &[
    // Front (+Z)
    Vertex::new((-1.0, 1.0, 1.0), (1.0, 1.0, 1.0), (0.0, 0.0, 1.0)),
    Vertex::new((1.0, 1.0, 1.0), (1.0, 1.0, 1.0), (0.0, 0.0, 1.0)),
    Vertex::new((-1.0, -1.0, 1.0), (1.0, 1.0, 1.0), (0.0, 0.0, 1.0)),
    Vertex::new((1.0, -1.0, 1.0), (1.0, 1.0, 1.0), (0.0, 0.0, 1.0)),
    // Back (-Z)
    Vertex::new((-1.0, 1.0, -1.0), (1.0, 1.0, 1.0), (0.0, 0.0, -1.0)),
    Vertex::new((1.0, 1.0, -1.0), (1.0, 1.0, 1.0), (0.0, 0.0, -1.0)),
    Vertex::new((-1.0, -1.0, -1.0), (1.0, 1.0, 1.0), (0.0, 0.0, -1.0)),
    Vertex::new((1.0, -1.0, -1.0), (1.0, 1.0, 1.0), (0.0, 0.0, -1.0)),
    // Left (-X)
    Vertex::new((-1.0, 1.0, -1.0), (1.0, 1.0, 1.0), (-1.0, 0.0, 0.0)),
    Vertex::new((-1.0, 1.0, 1.0), (1.0, 1.0, 1.0), (-1.0, 0.0, 0.0)),
    Vertex::new((-1.0, -1.0, -1.0), (1.0, 1.0, 1.0), (-1.0, 0.0, 0.0)),
    Vertex::new((-1.0, -1.0, 1.0), (1.0, 1.0, 1.0), (-1.0, 0.0, 0.0)),
    // Right (+X)
    Vertex::new((1.0, 1.0, 1.0), (1.0, 1.0, 1.0), (1.0, 0.0, 0.0)),
    Vertex::new((1.0, 1.0, -1.0), (1.0, 1.0, 1.0), (1.0, 0.0, 0.0)),
    Vertex::new((1.0, -1.0, 1.0), (1.0, 1.0, 1.0), (1.0, 0.0, 0.0)),
    Vertex::new((1.0, -1.0, -1.0), (1.0, 1.0, 1.0), (1.0, 0.0, 0.0)),
    // Top (+Y)
    Vertex::new((-1.0, 1.0, -1.0), (1.0, 1.0, 1.0), (0.0, 1.0, 0.0)),
    Vertex::new((1.0, 1.0, -1.0), (1.0, 1.0, 1.0), (0.0, 1.0, 0.0)),
    Vertex::new((-1.0, 1.0, 1.0), (1.0, 1.0, 1.0), (0.0, 1.0, 0.0)),
    Vertex::new((1.0, 1.0, 1.0), (1.0, 1.0, 1.0), (0.0, 1.0, 0.0)),
    // Bottom (-Y)
    Vertex::new((-1.0, -1.0, 1.0), (1.0, 1.0, 1.0), (0.0, -1.0, 0.0)),
    Vertex::new((1.0, -1.0, 1.0), (1.0, 1.0, 1.0), (0.0, -1.0, 0.0)),
    Vertex::new((-1.0, -1.0, -1.0), (1.0, 1.0, 1.0), (0.0, -1.0, 0.0)),
    Vertex::new((1.0, -1.0, -1.0), (1.0, 1.0, 1.0), (0.0, -1.0, 0.0)),
];
pub(in crate::update::render_objects::new) const CUBE_INDICES: &[u32] = &[
    0, 1, 2, 1, 3, 2, // Front
    4, 6, 5, 5, 6, 7, // Back
    8, 9, 10, 9, 11, 10, // Left
    12, 13, 14, 13, 15, 14, // Right
    16, 17, 18, 17, 19, 18, // Top
    20, 22, 21, 21, 22, 23, // Bottom
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
            vertices.push(Vertex::new(position, color, -Vector3f::Z));
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

/// Generate a cylinder primitive mesh
pub(in crate::update::render_objects::new) fn cylinder() -> (Vec<Vertex>, Vec<u32>) {
    const SEGMENTS: usize = 32;
    const HEIGHT: f32 = 2.0;

    let mut vertices = Vec::with_capacity(SEGMENTS * 4 + 2);
    let mut indices = Vec::with_capacity(SEGMENTS * 12);

    let color = (1.0, 1.0, 1.0);

    // Side vertices (radial normals)
    for i in 0..SEGMENTS {
        let angle = (i as f32 / SEGMENTS as f32) * std::f32::consts::TAU;
        let x = angle.cos();
        let z = angle.sin();
        let normal = (x, 0.0, z);
        vertices.push(Vertex::new((x, HEIGHT / 2.0, z), color, normal));
        vertices.push(Vertex::new((x, -HEIGHT / 2.0, z), color, normal));
    }

    // Top cap ring (+Y normal)
    let top_ring_start = vertices.len() as u32;
    for i in 0..SEGMENTS {
        let angle = (i as f32 / SEGMENTS as f32) * std::f32::consts::TAU;
        let x = angle.cos();
        let z = angle.sin();
        vertices.push(Vertex::new((x, HEIGHT / 2.0, z), color, (0.0, 1.0, 0.0)));
    }

    // Bottom cap ring (-Y normal)
    let bottom_ring_start = vertices.len() as u32;
    for i in 0..SEGMENTS {
        let angle = (i as f32 / SEGMENTS as f32) * std::f32::consts::TAU;
        let x = angle.cos();
        let z = angle.sin();
        vertices.push(Vertex::new((x, -HEIGHT / 2.0, z), color, (0.0, -1.0, 0.0)));
    }

    // Top and bottom center vertices
    let top_center = vertices.len() as u32;
    vertices.push(Vertex::new(
        (0.0, HEIGHT / 2.0, 0.0),
        color,
        (0.0, 1.0, 0.0),
    ));
    let bottom_center = vertices.len() as u32;
    vertices.push(Vertex::new(
        (0.0, -HEIGHT / 2.0, 0.0),
        color,
        (0.0, -1.0, 0.0),
    ));

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
        indices.push(top_center);
        indices.push(top_ring_start + i as u32);
        indices.push(top_ring_start + next as u32);

        // Bottom face
        indices.push(bottom_center);
        indices.push(bottom_ring_start + next as u32);
        indices.push(bottom_ring_start + i as u32);
    }

    (vertices, indices)
}
