use crate::graphics::DisplayMode;

#[cfg(debug_assertions)]
impl Default for DisplayMode {
    fn default() -> Self {
        DisplayMode::Resizable
    }
}

#[cfg(not(debug_assertions))]
impl Default for DisplayMode {
    fn default() -> Self {
        DisplayMode::Windowed
    }
}
