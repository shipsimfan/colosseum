use crate::input::KeyCode;

impl KeyCode {
    /// Gets a string representation of this keycode
    pub const fn as_str(&self) -> &'static str {
        match self {
            KeyCode::UpArrow => "↑",
            KeyCode::DownArrow => "↓",
            KeyCode::LeftArrow => "←",
            KeyCode::RightArrow => "→",
            KeyCode::LeftShift => "⇧L",
            KeyCode::RightShift => "⇧R",
            KeyCode::A => "A",
            KeyCode::B => "B",
            KeyCode::C => "C",
            KeyCode::D => "D",
            KeyCode::E => "E",
            KeyCode::F => "F",
            KeyCode::G => "G",
            KeyCode::H => "H",
            KeyCode::I => "I",
            KeyCode::J => "J",
            KeyCode::K => "K",
            KeyCode::L => "L",
            KeyCode::M => "M",
            KeyCode::N => "N",
            KeyCode::O => "O",
            KeyCode::P => "P",
            KeyCode::Q => "Q",
            KeyCode::R => "R",
            KeyCode::S => "S",
            KeyCode::T => "T",
            KeyCode::U => "U",
            KeyCode::V => "V",
            KeyCode::W => "W",
            KeyCode::X => "X",
            KeyCode::Y => "Y",
            KeyCode::Z => "Z",
        }
    }
}
