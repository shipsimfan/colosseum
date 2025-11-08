use crate::math::{number::IntoF32, Rational};

impl<T: IntoF32 + Clone> std::fmt::Display for Rational<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.clone().into_f32().fmt(f)
    }
}
