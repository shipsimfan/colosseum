use crate::graphics::{DirectionalLight, DirectionalLightHandle, DirectionalLights};
use std::ops::{Index, IndexMut};

impl Index<DirectionalLightHandle> for DirectionalLights {
    type Output = DirectionalLight;

    fn index(&self, index: DirectionalLightHandle) -> &Self::Output {
        &self.list[index]
    }
}

impl IndexMut<DirectionalLightHandle> for DirectionalLights {
    fn index_mut(&mut self, index: DirectionalLightHandle) -> &mut Self::Output {
        &mut self.list[index]
    }
}
