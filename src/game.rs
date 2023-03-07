use crate::{Error, GraphicsSettings, InitialScene, Scene, Window};

pub struct Game {
    window: Window,

    scene: Box<dyn Scene>,
}

impl Game {
    pub fn new<S: InitialScene>(
        graphics_settings: GraphicsSettings,
        context: S::Context,
    ) -> Result<Self, Error> {
        let mut window = Window::new(graphics_settings)?;

        let scene = S::new(context, &mut window);

        Ok(Game { window, scene })
    }

    pub fn run() {}
}
