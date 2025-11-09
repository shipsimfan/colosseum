mod as_str;
mod display;
mod from_button;
mod from_scan_code;
mod into;

/// A code representing a key on a keyboard
#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum KeyCode {
    /// The up arrow key
    UpArrow = 0,

    /// The down arrow key
    DownArrow = 1,

    /// The left arrow key
    LeftArrow = 2,

    /// The right arrow key
    RightArrow = 3,

    /// The left shift key
    LeftShift = 4,

    /// The right shift key
    RightShift = 5,

    /// The letter "A"
    A = 65,

    /// The letter "B"
    B = 66,

    /// The letter "C"
    C = 67,

    /// The letter "D"
    D = 68,

    /// The letter "E"
    E = 69,

    /// The letter "F"
    F = 70,

    /// The letter "G"
    G = 71,

    /// The letter "H"
    H = 72,

    /// The letter "I"
    I = 73,

    /// The letter "J"
    J = 74,

    /// The letter "K"
    K = 75,

    /// The letter "L"
    L = 76,

    /// The letter "M"
    M = 77,

    /// The letter "N"
    N = 78,

    /// The letter "O"
    O = 79,

    /// The letter "P"
    P = 80,

    /// The letter "Q"
    Q = 81,

    /// The letter "R"
    R = 82,

    /// The letter "S"
    S = 83,

    /// The letter "T"
    T = 84,

    /// The letter "U"
    U = 85,

    /// The letter "V"
    V = 86,

    /// The letter "W"
    W = 87,

    /// The letter "X"
    X = 88,

    /// The letter "Y"
    Y = 89,

    /// The letter "Z"
    Z = 90,
}
