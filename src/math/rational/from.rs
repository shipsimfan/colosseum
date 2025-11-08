use crate::math::{
    number::{One, Zero},
    Rational,
};

impl<T> From<(T, T)> for Rational<T> {
    fn from((x, y): (T, T)) -> Self {
        Rational::new(x, y)
    }
}

impl<T> From<[T; 2]> for Rational<T> {
    fn from(a: [T; 2]) -> Self {
        Rational::from_array(a)
    }
}

impl<T: Clone> From<&[T]> for Rational<T> {
    fn from(s: &[T]) -> Self {
        Rational::from_slice(s)
    }
}

impl<T: Clone> From<&[T; 2]> for Rational<T> {
    fn from(s: &[T; 2]) -> Self {
        Rational::from_slice(s)
    }
}

impl<T: One> From<T> for Rational<T> {
    fn from(v: T) -> Self {
        Rational::integer(v)
    }
}

impl<T: One + Zero> From<()> for Rational<T> {
    fn from(_: ()) -> Self {
        Rational::zero()
    }
}
