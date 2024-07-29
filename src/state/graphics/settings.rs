use crate::settings;
use data_format::{Deserialize, Serialize};

/// The graphics settings
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Settings {
    /// The desired window width
    width: usize,

    /// The desired window height
    height: usize,

    /// The desired device to use for rendering (usually a GPU)
    device: Option<String>,
}

impl Settings {
    /// Gets the desired width of the window
    pub fn width(&self) -> usize {
        self.width
    }

    /// Gets the desired height of the window
    pub fn height(&self) -> usize {
        self.height
    }

    /// Gets the desired device to use for rendering
    pub fn device(&self) -> Option<&str> {
        self.device.as_ref().map(|str| str.as_str())
    }

    /// Sets the desired device for rendering
    pub fn set_device(&mut self, device: String) {
        self.device = Some(device);
    }
}

impl settings::Settings for Settings {
    const NAME: &str = "graphics";
}

impl Default for Settings {
    fn default() -> Self {
        Settings {
            width: 1280,
            height: 720,
            device: None,
        }
    }
}
