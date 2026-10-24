use crate::render::Vertex;
use alexandria::math::Vector3f;

/// Create a new plane primitive mesh
pub(in crate::render::public_data::primitives::new) fn plane() -> (Vec<Vertex>, Vec<u32>) {
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
