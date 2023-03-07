struct Scene {}

fn main() {
    run()
        .map_err(|error| {
            colosseum::MessageBox::run(
                colosseum::MessageBoxClass::Error,
                "Unable to launch game",
                &error.to_string(),
            )
        })
        .ok();
}

fn run() -> Result<(), colosseum::Error> {
    let game = colosseum::Game::new::<Scene>(colosseum::GraphicsSettings::default(), ())?;

    Ok(game.run())
}

impl colosseum::InitialScene for Scene {
    type Context = ();

    fn new(_: Self::Context, _: &mut colosseum::Window) -> Box<dyn colosseum::Scene> {
        Box::new(Scene {})
    }
}

impl colosseum::Scene for Scene {}

/*
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

    fn new(
        window: &mut colosseum::Window<Self::Input>,
        ui_root: &std::rc::Rc<colosseum::ui::Element<Self>>,
    ) -> Self {
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

        let image = colosseum::ui::Element::<Self>::new(
            "Image".to_owned(),
            colosseum::ui::Image::new(None),
        );

        let mut image_transform = image.transform_mut();
        image_transform
            .set_anchors_reference(colosseum::ui::VAlign::Top, colosseum::ui::HAlign::Left);
        image_transform.set_scale(colosseum::Vector2::new(0.25, 0.25));
        image_transform.set_position(colosseum::Vector2::new(0.01, -0.01));
        drop(image_transform);

        let mut image2 = colosseum::ui::Image::new(None);
        image2.set_tint(colosseum::Vector4::new(0.8, 0.2, 0.2, 1.0));
        let image2 = colosseum::ui::Element::<Self>::new("Image2".to_owned(), image2);
        let mut image_transform = image2.transform_mut();
        image_transform
            .set_anchors_reference(colosseum::ui::VAlign::Top, colosseum::ui::HAlign::Right);
        image_transform.set_scale(colosseum::Vector2::new(0.1, 0.1));
        image_transform.set_position(colosseum::Vector2::new(-0.01, -0.01));
        drop(image_transform);

        image.add_child(image2);

        ui_root.add_child(image);

        Game {
            camera: colosseum::Camera::new(window),
            mesh_renderer,
        }
    }

    fn update(
        &mut self,
        delta_time: f32,
        _: &std::rc::Rc<colosseum::ui::Element<Self>>,
        window: &mut colosseum::Window<Self::Input>,
    ) {
        let mut x_position = self.mesh_renderer.transform().position().x();
        let mut y_position = self.mesh_renderer.transform().position().y();
        let mut z_position = self.mesh_renderer.transform().position().z();

        if window.input().get_key(colosseum::Key::Q) {
            y_position += delta_time;
        }

        if window.input().get_key(colosseum::Key::E) {
            y_position -= delta_time;
        }

        if window.input().get_key(colosseum::Key::LeftShift)
            || window.input().get_key(colosseum::Key::RightShift)
        {
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
                z_position += delta_time;
            }

            if window.input().get_key(colosseum::Key::S)
                || window.input().get_key(colosseum::Key::DownArrow)
            {
                z_position -= delta_time;
            }
        } else {
            let mut x_rotation = self.mesh_renderer.transform().rotation().x();
            let mut y_rotation = self.mesh_renderer.transform().rotation().y();

            if window.input().get_key(colosseum::Key::D)
                || window.input().get_key(colosseum::Key::RightArrow)
            {
                y_rotation += delta_time;
            }

            if window.input().get_key(colosseum::Key::A)
                || window.input().get_key(colosseum::Key::LeftArrow)
            {
                y_rotation -= delta_time;
            }

            if window.input().get_key(colosseum::Key::W)
                || window.input().get_key(colosseum::Key::UpArrow)
            {
                x_rotation -= delta_time;
            }

            if window.input().get_key(colosseum::Key::S)
                || window.input().get_key(colosseum::Key::DownArrow)
            {
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
*/
