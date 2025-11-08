use crate::graphics::DisplayMode;

impl std::fmt::Display for DisplayMode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            DisplayMode::Resizable => "resizable",
            DisplayMode::Windowed => "windowed",
            DisplayMode::Borderless => "borderless",
        }
        .fmt(f)
    }
}
