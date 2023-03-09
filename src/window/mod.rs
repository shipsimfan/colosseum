use crate::{Error, GraphicsSettings, Result};

mod create;

pub struct Window {
    instance: alexandria::Instance,
    window: alexandria::Window,
}

macro_rules! map_err {
    ($func:expr) => {
        $func.map_err(|_| Error::WindowCreationError)
    };
}

impl Window {
    pub(crate) fn new(title: &str, graphics_settings: GraphicsSettings) -> Result<Self> {
        let mut instance = map_err!(alexandria::Instance::new())?;

        let (display, adapter) = map_err!(create::get_display(
            &mut instance,
            graphics_settings.adapter(),
            graphics_settings.display(),
        ))?;

        let window = map_err!(alexandria::Window::new(
            title,
            graphics_settings
                .resolution()
                .map(|resolution| resolution.into_inner()),
            graphics_settings
                .refresh_rate()
                .map(|refresh_rate| refresh_rate.into_inner()),
            &mut instance,
            Some(adapter),
            Some(display),
        ))?;

        Ok(Window { instance, window })
    }

    pub(crate) fn poll_events(&mut self) -> bool {
        self.window.poll_events();
        self.window.is_alive()
    }
}
