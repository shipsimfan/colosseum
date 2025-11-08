use data_format::{Deserialize, Serialize};

mod client_to_window;
mod default;
mod display;
mod style;

/// The mode a window should be displayed with
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum DisplayMode {
    /// The window has a thick border and is resizable by it
    #[default]
    Resizable,

    /// The window has a thin border and is not resizable
    Windowed,

    /// The window has no border
    Borderless,
}
