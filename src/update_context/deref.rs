use crate::{ManagedObjects, UpdateContext};
use std::ops::{Deref, DerefMut};

impl<'a, Game: crate::Game> Deref for UpdateContext<'a, Game> {
    type Target = ManagedObjects;

    fn deref(&self) -> &Self::Target {
        self.objects
    }
}

impl<'a, Game: crate::Game> DerefMut for UpdateContext<'a, Game> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.objects
    }
}
