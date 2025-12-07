use crate::graphics::{MeshRenderer, MeshRendererHandle, MeshRenderers};
use std::ops::{Index, IndexMut};

impl Index<MeshRendererHandle> for MeshRenderers {
    type Output = MeshRenderer;

    fn index(&self, index: MeshRendererHandle) -> &Self::Output {
        &self.arena[index]
    }
}

impl IndexMut<MeshRendererHandle> for MeshRenderers {
    fn index_mut(&mut self, index: MeshRendererHandle) -> &mut Self::Output {
        &mut self.arena[index]
    }
}
