use crate::math::Quaternion;
use std::ops::{Index, IndexMut};

impl<T> Quaternion<T> {
    /// Returns a reference to an element
    pub const fn get(&self, idx: usize) -> Option<&T> {
        match idx {
            0 => Some(&self.x),
            1 => Some(&self.y),
            2 => Some(&self.z),
            3 => Some(&self.w),
            _ => None,
        }
    }

    /// Returns a mutable reference to an element
    pub const fn get_mut(&mut self, idx: usize) -> Option<&mut T> {
        match idx {
            0 => Some(&mut self.x),
            1 => Some(&mut self.y),
            2 => Some(&mut self.z),
            3 => Some(&mut self.w),
            _ => None,
        }
    }
}

impl<T> Index<usize> for Quaternion<T> {
    type Output = T;

    fn index(&self, index: usize) -> &Self::Output {
        match index {
            0 => &self.x,
            1 => &self.y,
            2 => &self.z,
            3 => &self.w,
            _ => panic!(
                "index out of bounds: the len is 4 but the index is {}",
                index
            ),
        }
    }
}

impl<T> IndexMut<usize> for Quaternion<T> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        match index {
            0 => &mut self.x,
            1 => &mut self.y,
            2 => &mut self.z,
            3 => &mut self.w,
            _ => panic!(
                "index out of bounds: the len is 4 but the index is {}",
                index
            ),
        }
    }
}
