use crate::render::Vertex;

/// Create a new cylinder primitive mesh
pub(in crate::render::public_data::primitives::new) fn cylinder() -> (Vec<Vertex>, Vec<u32>) {
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
