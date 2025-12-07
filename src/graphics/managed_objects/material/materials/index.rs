use crate::graphics::{Material, MaterialHandle, Materials};
use std::ops::{Index, IndexMut};

impl Index<MaterialHandle> for Materials {
    type Output = Material;

    fn index(&self, index: MaterialHandle) -> &Self::Output {
        &self.arena[index]
    }
}

impl IndexMut<MaterialHandle> for Materials {
    fn index_mut(&mut self, index: MaterialHandle) -> &mut Self::Output {
        &mut self.arena[index]
    }
}
