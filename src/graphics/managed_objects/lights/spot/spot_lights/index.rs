use crate::graphics::{SpotLight, SpotLightHandle, SpotLights};
use std::ops::{Index, IndexMut};

impl Index<SpotLightHandle> for SpotLights {
    type Output = SpotLight;

    fn index(&self, index: SpotLightHandle) -> &Self::Output {
        &self.list[index]
    }
}

impl IndexMut<SpotLightHandle> for SpotLights {
    fn index_mut(&mut self, index: SpotLightHandle) -> &mut Self::Output {
        &mut self.list[index]
    }
}
