use std::f32::consts::PI;

pub struct Game {
    camera: colosseum::Camera,
    sprite: colosseum::Sprite,
}

impl colosseum::Game for Game {
    const INITIAL_TITLE: &'static str = "Sprite Example";

    fn new(window: &mut colosseum::Window<Self::Input>) -> Self {
        let mut camera = colosseum::Camera::new(window);

        camera.set_projection(
            colosseum::Projection::orthographic(10.0, -1.0, 1000.0),
            window,
        );

        let texture = colosseum::Texture::load(
            "./examples/sprite.qoi",
            colosseum::SampleType::Point,
            window,
        );

        Game {
            camera,
            sprite: colosseum::Sprite::new(window, Some(texture)),
        }
    }

    fn update(&mut self, delta_time: f32, window: &mut colosseum::Window<Self::Input>) {
        let mut x_position = self.sprite.transform().position().x();
        let mut y_position = self.sprite.transform().position().y();
        let mut rotation = self.sprite.transform().rotation();

        if window.input().get_key(b'D') || window.input().get_key(0x27) {
            x_position += delta_time;
        }

        if window.input().get_key(b'A') || window.input().get_key(0x25) {
            x_position -= delta_time;
        }

        if window.input().get_key(b'W') || window.input().get_key(0x26) {
            y_position += delta_time;
        }

        if window.input().get_key(b'S') || window.input().get_key(0x28) {
            y_position -= delta_time;
        }

        if window.input().get_key(b'Q') {
            rotation -= delta_time;
        }

        if window.input().get_key(b'E') {
            rotation += delta_time;
        }

        if x_position < -4.5 {
            x_position = -4.5;
        } else if x_position > 4.5 {
            x_position = 4.5;
        }

        let y_max = 4.5 / window.aspect();
        if y_position < -y_max {
            y_position = -y_max;
        } else if y_position > y_max {
            y_position = y_max;
        }

        if rotation > 2.0 * PI {
            rotation -= 2.0 * PI;
        } else if rotation < 2.0 * PI {
            rotation += 2.0 * PI;
        }

        self.sprite
            .transform_mut()
            .set_position(colosseum::Vector2::new(x_position, y_position));
        self.sprite.transform_mut().set_rotation(rotation);
    }

    fn render(&mut self, window: &mut colosseum::Window<Self::Input>) {
        self.camera.set_active(window);
        self.sprite.render(window);
    }

    fn clear_color(&self) -> [f32; 4] {
        [0.0, 0.0, 0.0, 1.0]
    }
}

fn main() {
    colosseum::App::<Game>::new();
}
