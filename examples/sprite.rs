fn main() {}

/*
use std::f32::consts::PI;

pub struct Game {
    camera: colosseum::Camera,
    sprite: colosseum::Sprite,
}

impl colosseum::Game for Game {
    const INITIAL_TITLE: &'static str = "Sprite Example";

    fn new(
        window: &mut colosseum::Window<Self::Input>,
        _: &std::rc::Rc<colosseum::ui::Element<Self>>,
    ) -> Self {
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
            sprite: colosseum::Sprite::new(Some(texture)),
        }
    }

    fn update(
        &mut self,
        delta_time: f32,
        _: &std::rc::Rc<colosseum::ui::Element<Self>>,
        window: &mut colosseum::Window<Self::Input>,
    ) {
        let mut x_position = self.sprite.transform().position().x();
        let mut y_position = self.sprite.transform().position().y();
        let mut rotation = self.sprite.transform().rotation();

        if window.input().get_key(colosseum::Key::D)
            || window.input().get_key(colosseum::Key::RightArrow)
        {
            x_position += delta_time;
        }

        if window.input().get_key(colosseum::Key::A)
            || window.input().get_key(colosseum::Key::LeftArrow)
        {
            x_position -= delta_time;
        }

        if window.input().get_key(colosseum::Key::W)
            || window.input().get_key(colosseum::Key::UpArrow)
        {
            y_position += delta_time;
        }

        if window.input().get_key(colosseum::Key::S)
            || window.input().get_key(colosseum::Key::DownArrow)
        {
            y_position -= delta_time;
        }

        if window.input().get_key(colosseum::Key::Q) {
            rotation -= delta_time;
        }

        if window.input().get_key(colosseum::Key::E) {
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
*/
