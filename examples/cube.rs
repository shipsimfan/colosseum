use std::f32::consts::PI;

struct Game {
    camera: colosseum::Camera,
    mesh_renderer: colosseum::MeshRenderer,
}

fn main() {
    colosseum::App::<Game>::new();
}

impl colosseum::Game for Game {
    const INITIAL_TITLE: &'static str = "Cube Example";

    fn new(window: &mut colosseum::Window<Self::Input>) -> Self {
        let mesh = colosseum::Mesh::new(
            &[
                colosseum::Vertex::new(1.0, -1.0, 1.0, 1.0, 0.0, 0.0, 1.0, 0.0, 0.0),
                colosseum::Vertex::new(1.0, -1.0, -1.0, 0.0, 1.0, 0.0, 1.0, 0.0, 0.0),
                colosseum::Vertex::new(1.0, 1.0, -1.0, 0.0, 0.0, 1.0, 1.0, 0.0, 0.0),
                colosseum::Vertex::new(1.0, 1.0, 1.0, 1.0, 1.0, 0.0, 1.0, 0.0, 0.0),
                colosseum::Vertex::new(-1.0, -1.0, 1.0, 1.0, 0.0, 1.0, 1.0, 0.0, 0.0),
                colosseum::Vertex::new(-1.0, -1.0, -1.0, 0.0, 1.0, 1.0, 1.0, 0.0, 0.0),
                colosseum::Vertex::new(-1.0, 1.0, -1.0, 1.0, 1.0, 1.0, 1.0, 0.0, 0.0),
                colosseum::Vertex::new(-1.0, 1.0, 1.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0),
            ],
            &[
                4, 0, 3, 4, 3, 7, 0, 1, 2, 0, 2, 3, 1, 5, 6, 1, 6, 2, 5, 4, 7, 5, 7, 6, 7, 3, 2, 7,
                2, 6, 0, 5, 1, 0, 4, 5,
            ],
            window,
        );

        let mut mesh_renderer = colosseum::MeshRenderer::new(mesh, None);

        mesh_renderer
            .transform_mut()
            .set_position(colosseum::Vector3::new(0.0, 0.0, 10.0));

        Game {
            camera: colosseum::Camera::new(window),
            mesh_renderer,
        }
    }

    fn update(&mut self, delta_time: f32, window: &mut colosseum::Window<Self::Input>) {
        let mut x_position = self.mesh_renderer.transform().position().x();
        let mut y_position = self.mesh_renderer.transform().position().y();
        let mut z_position = self.mesh_renderer.transform().position().z();

        if window.input().get_key(b'Q') {
            y_position += delta_time;
        }

        if window.input().get_key(b'E') {
            y_position -= delta_time;
        }

        if window.input().get_key(0x10) {
            if window.input().get_key(b'D') || window.input().get_key(0x27) {
                x_position += delta_time;
            }

            if window.input().get_key(b'A') || window.input().get_key(0x25) {
                x_position -= delta_time;
            }

            if window.input().get_key(b'W') || window.input().get_key(0x26) {
                z_position += delta_time;
            }

            if window.input().get_key(b'S') || window.input().get_key(0x28) {
                z_position -= delta_time;
            }
        } else {
            let mut x_rotation = self.mesh_renderer.transform().rotation().x();
            let mut y_rotation = self.mesh_renderer.transform().rotation().y();

            if window.input().get_key(b'D') || window.input().get_key(0x27) {
                y_rotation += delta_time;
            }

            if window.input().get_key(b'A') || window.input().get_key(0x25) {
                y_rotation -= delta_time;
            }

            if window.input().get_key(b'W') || window.input().get_key(0x26) {
                x_rotation -= delta_time;
            }

            if window.input().get_key(b'S') || window.input().get_key(0x28) {
                x_rotation += delta_time;
            }

            if y_rotation < -2.0 * PI {
                y_rotation += 2.0 * PI;
            }

            if y_rotation > 2.0 * PI {
                y_rotation -= 2.0 * PI;
            }

            if x_rotation < -2.0 * PI {
                x_rotation += 2.0 * PI;
            }

            if x_rotation > 2.0 * PI {
                x_rotation -= 2.0 * PI;
            }

            self.mesh_renderer
                .transform_mut()
                .set_rotation(colosseum::Vector3::new(x_rotation, y_rotation, 0.0));
        }

        self.mesh_renderer
            .transform_mut()
            .set_position(colosseum::Vector3::new(x_position, y_position, z_position));
    }

    fn render(&mut self, window: &mut colosseum::Window<Self::Input>) {
        self.camera.set_active(window);

        self.mesh_renderer.render(window);
    }

    fn clear_color(&self) -> [f32; 4] {
        [0.0, 0.0, 0.0, 1.0]
    }
}
