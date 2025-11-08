use crate::math::{number::IntoF32, Rational};

impl<T> Into<(T, T)> for Rational<T> {
    fn into(self) -> (T, T) {
        (self.numerator, self.denominator)
    }
}

impl<T> Into<[T; 2]> for Rational<T> {
    fn into(self) -> [T; 2] {
        [self.numerator, self.denominator]
    }
}

impl<T: IntoF32> IntoF32 for Rational<T> {
    fn into_f32(self) -> f32 {
        self.numerator.into_f32() / self.denominator.into_f32()
    }
}

impl<T: IntoF32> Into<f32> for Rational<T> {
    fn into(self) -> f32 {
        self.into_f32()
    }
}
