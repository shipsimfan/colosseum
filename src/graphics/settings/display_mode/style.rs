use crate::graphics::DisplayMode;
use win32::{
    DWORD, WS_BORDER, WS_CAPTION, WS_EX_APPWINDOW, WS_MINIMIZEBOX, WS_OVERLAPPEDWINDOW, WS_POPUP,
    WS_SYSMENU, WS_VISIBLE,
};

impl DisplayMode {
    /// Gets the style and extended style for this [`DisplayMode`]
    pub(crate) const fn style(&self) -> (DWORD, DWORD) {
        match self {
            DisplayMode::Resizable => (WS_OVERLAPPEDWINDOW | WS_VISIBLE, 0),
            DisplayMode::Windowed => (
                WS_BORDER | WS_CAPTION | WS_SYSMENU | WS_MINIMIZEBOX | WS_VISIBLE,
                0,
            ),
            DisplayMode::Borderless => (WS_POPUP | WS_VISIBLE, WS_EX_APPWINDOW),
        }
    }
}
