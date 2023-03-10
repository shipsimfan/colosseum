use crate::{Error, GraphicsSettings, Result};

mod create;

pub struct Window {
    instance: alexandria::Instance,
    window: alexandria::Window,

    log_debug_messages: bool,
}

macro_rules! map_err {
    ($func:expr) => {
        $func.map_err(|_| Error::WindowCreationError)
    };
}

impl Window {
    pub(crate) fn new(
        title: &str,
        graphics_settings: GraphicsSettings,
        log_debug_messages: bool,
    ) -> Result<Self> {
        let mut instance = map_err!(alexandria::Instance::new(log_debug_messages))?;

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

        Ok(Window {
            instance,
            window,
            log_debug_messages,
        })
    }

    pub(crate) fn poll_events(&mut self) -> bool {
        if self.log_debug_messages {
            self.log_debug_messages();
        }

        self.window.poll_events();
        self.window.is_alive()
    }

    fn log_debug_messages(&mut self) {
        while let Some(message) = self.instance.pop_debug_message().unwrap() {
            println!("[Colosseum][{}] {}", message.level(), message.message());
        }
    }
}
