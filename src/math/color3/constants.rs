use crate::math::{
    number::{One, Zero},
    Color3,
};

impl<T: Zero> Color3<T> {
    /// Value which represents black
    pub const BLACK: Color3<T> = Color3::black();
}

impl<T: One> Color3<T> {
    /// Value which represents white
    pub const WHITE: Color3<T> = Color3::white();
}

impl<T: Zero + One> Color3<T> {
    /// Value which represents red
    pub const RED: Color3<T> = Color3::red();

    /// Value which represents green
    pub const GREEN: Color3<T> = Color3::green();

    /// Value which represents blue
    pub const BLUE: Color3<T> = Color3::blue();
}
