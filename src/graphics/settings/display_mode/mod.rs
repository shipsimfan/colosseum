use data_format::{Deserialize, Serialize};

mod client_to_window;
mod default;
mod display;
mod from_w_param;
mod style;

/// The mode a window should be displayed with
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum DisplayMode {
    /// The window has a thick border and is resizable by it
    Resizable = 0,

    /// The window has a thin border and is not resizable
    Windowed = 1,

    /// The window has no border
    Borderless = 2,
}
