use crate::math::Matrix4x4;
use std::ops::{Index, IndexMut};

impl<T> Matrix4x4<T> {
    /// Returns a reference to a row
    pub fn row(&self, row: usize) -> Option<&[T; 4]> {
        self.v.get(row)
    }

    /// Returns a reference to an element
    pub fn get(&self, row: usize, column: usize) -> Option<&T> {
        self.row(row).and_then(|row| row.get(column))
    }

    /// Returns a mutable reference to a row
    pub fn row_mut(&mut self, row: usize) -> Option<&mut [T; 4]> {
        self.v.get_mut(row)
    }

    /// Returns a mutable reference to an element
    pub fn get_mut(&mut self, row: usize, column: usize) -> Option<&mut T> {
        self.row_mut(row).and_then(|row| row.get_mut(column))
    }
}

impl<T> Index<usize> for Matrix4x4<T> {
    type Output = [T; 4];

    fn index(&self, index: usize) -> &Self::Output {
        &self.v[index]
    }
}

impl<T> IndexMut<usize> for Matrix4x4<T> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.v[index]
    }
}

impl<T> Index<(usize, usize)> for Matrix4x4<T> {
    type Output = T;

    fn index(&self, index: (usize, usize)) -> &Self::Output {
        &self.v[index.0][index.1]
    }
}

impl<T> IndexMut<(usize, usize)> for Matrix4x4<T> {
    fn index_mut(&mut self, index: (usize, usize)) -> &mut Self::Output {
        &mut self.v[index.0][index.1]
    }
}
