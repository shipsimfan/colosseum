use crate::input::KeyCode;

impl KeyCode {
    /// Convert a `button` into a [`KeyCode`]
    pub fn from_button(button: u8) -> Option<Self> {
        Some(match button {
            0 => KeyCode::UpArrow,
            1 => KeyCode::DownArrow,
            2 => KeyCode::LeftArrow,
            3 => KeyCode::RightArrow,
            4 => KeyCode::LeftShift,
            5 => KeyCode::RightShift,
            65 => KeyCode::A,
            66 => KeyCode::B,
            67 => KeyCode::C,
            68 => KeyCode::D,
            69 => KeyCode::E,
            70 => KeyCode::F,
            71 => KeyCode::G,
            72 => KeyCode::H,
            73 => KeyCode::I,
            74 => KeyCode::J,
            75 => KeyCode::K,
            76 => KeyCode::L,
            77 => KeyCode::M,
            78 => KeyCode::N,
            79 => KeyCode::O,
            80 => KeyCode::P,
            81 => KeyCode::Q,
            82 => KeyCode::R,
            83 => KeyCode::S,
            84 => KeyCode::T,
            85 => KeyCode::U,
            86 => KeyCode::V,
            87 => KeyCode::W,
            88 => KeyCode::X,
            89 => KeyCode::Y,
            90 => KeyCode::Z,
            _ => return None,
        })
    }
}
