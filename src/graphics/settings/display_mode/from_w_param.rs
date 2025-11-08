use crate::graphics::DisplayMode;
use win32::WPARAM;

impl DisplayMode {
    /// Convert a [`WPARAM`] into a [`DisplayMode`]
    pub(crate) fn from_w_param(w_param: WPARAM) -> Option<Self> {
        Some(match w_param {
            0 => DisplayMode::Resizable,
            1 => DisplayMode::Windowed,
            2 => DisplayMode::Borderless,
            _ => return None,
        })
    }
}
