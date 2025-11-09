use crate::input::KeyCode;

impl KeyCode {
    /// Convert `scan_code` into a [`KeyCode`]
    pub(crate) fn from_scan_code(scan_code: u16) -> Option<Self> {
        Some(match scan_code {
            16 => KeyCode::Q,
            17 => KeyCode::W,
            18 => KeyCode::E,
            19 => KeyCode::R,
            20 => KeyCode::T,
            21 => KeyCode::Y,
            22 => KeyCode::U,
            23 => KeyCode::I,
            24 => KeyCode::O,
            25 => KeyCode::P,
            30 => KeyCode::A,
            31 => KeyCode::S,
            32 => KeyCode::D,
            33 => KeyCode::F,
            34 => KeyCode::G,
            35 => KeyCode::H,
            36 => KeyCode::J,
            37 => KeyCode::K,
            38 => KeyCode::L,
            42 => KeyCode::LeftShift,
            44 => KeyCode::Z,
            45 => KeyCode::X,
            46 => KeyCode::C,
            47 => KeyCode::V,
            48 => KeyCode::B,
            49 => KeyCode::N,
            50 => KeyCode::M,
            54 => KeyCode::RightShift,
            328 => KeyCode::UpArrow,
            331 => KeyCode::LeftArrow,
            333 => KeyCode::RightArrow,
            336 => KeyCode::DownArrow,
            _ => return None,
        })
    }
}
