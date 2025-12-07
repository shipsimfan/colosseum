use crate::{
    graphics::context::{LightType, lights::LightList},
    util::Arena,
};
use std::ops::{Deref, DerefMut};

impl<T: LightType> Deref for LightList<T> {
    type Target = Arena<T>;

    fn deref(&self) -> &Self::Target {
        &self.arena
    }
}

impl<T: LightType> DerefMut for LightList<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.arena
    }
}
