use crate::math::Color3;
use std::ops::{Index, IndexMut};

impl<T> Color3<T> {
    /// Returns a reference to an element
    pub const fn get(&self, idx: usize) -> Option<&T> {
        match idx {
            0 => Some(&self.r),
            1 => Some(&self.g),
            2 => Some(&self.b),
            _ => None,
        }
    }

    /// Returns a mutable reference to an element
    pub const fn get_mut(&mut self, idx: usize) -> Option<&mut T> {
        match idx {
            0 => Some(&mut self.r),
            1 => Some(&mut self.g),
            2 => Some(&mut self.b),
            _ => None,
        }
    }
}

impl<T> Index<usize> for Color3<T> {
    type Output = T;

    fn index(&self, index: usize) -> &Self::Output {
        match index {
            0 => &self.r,
            1 => &self.g,
            2 => &self.b,
            _ => panic!(
                "index out of bounds: the len is 3 but the index is {}",
                index
            ),
        }
    }
}

impl<T> IndexMut<usize> for Color3<T> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        match index {
            0 => &mut self.r,
            1 => &mut self.g,
            2 => &mut self.b,
            _ => panic!(
                "index out of bounds: the len is 3 but the index is {}",
                index
            ),
        }
    }
}
