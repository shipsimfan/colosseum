use crate::graphics::{Camera, CameraHandle, Cameras};
use std::ops::{Index, IndexMut};

impl Index<CameraHandle> for Cameras {
    type Output = Camera;

    fn index(&self, index: CameraHandle) -> &Self::Output {
        &self.arena[index]
    }
}

impl IndexMut<CameraHandle> for Cameras {
    fn index_mut(&mut self, index: CameraHandle) -> &mut Self::Output {
        &mut self.arena[index]
    }
}
