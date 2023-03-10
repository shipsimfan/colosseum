use crate::{Error, GraphicsSettings, InitialScene, Scene, Window};

pub struct Game {
    window: Window,

    scene: Box<dyn Scene>,
}

impl Game {
    pub fn new<S: InitialScene>(
        title: &str,
        graphics_settings: GraphicsSettings,
        log_debug_messages: bool,
        context: S::Context,
    ) -> Result<Self, Error> {
        let mut window = Window::new(title, graphics_settings, log_debug_messages)?;

        let scene = S::new(context, &mut window);

        Ok(Game { window, scene })
    }

    pub fn run(mut self) {
        while self.window.poll_events() {
            match self.scene.update(&mut self.window) {
                Some(new_scene) => self.scene = new_scene,
                None => {}
            }

            self.scene.render(&mut self.window)
        }
    }
}
