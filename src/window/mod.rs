use crate::{Error, GraphicsSettings, Result};
mod create;

pub struct Window {
    instance: alexandria::Instance,
}

macro_rules! map_err {
    ($func:expr) => {
        $func.map_err(|_| Error::WindowCreationError)
    };
}

impl Window {
    pub(crate) fn new(graphics_settings: GraphicsSettings) -> Result<Self> {
        let mut instance = map_err!(alexandria::Instance::new())?;

        let display = map_err!(create::get_display(
            &mut instance,
            graphics_settings.adapter(),
            graphics_settings.display(),
        ))?;

        Ok(Window { instance })
    }
}
