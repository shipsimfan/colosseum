use crate::{Transform, TransformHandle, Transforms};
use std::ops::{Index, IndexMut};

impl Index<TransformHandle> for Transforms {
    type Output = Transform;

    fn index(&self, index: TransformHandle) -> &Self::Output {
        &self.arena[index]
    }
}

impl IndexMut<TransformHandle> for Transforms {
    fn index_mut(&mut self, index: TransformHandle) -> &mut Self::Output {
        &mut self.arena[index]
    }
}
