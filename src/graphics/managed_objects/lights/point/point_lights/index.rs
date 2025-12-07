use crate::graphics::{PointLight, PointLightHandle, PointLights};
use std::ops::{Index, IndexMut};

impl Index<PointLightHandle> for PointLights {
    type Output = PointLight;

    fn index(&self, index: PointLightHandle) -> &Self::Output {
        &self.list[index]
    }
}

impl IndexMut<PointLightHandle> for PointLights {
    fn index_mut(&mut self, index: PointLightHandle) -> &mut Self::Output {
        &mut self.list[index]
    }
}
