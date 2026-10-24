use crate::update::{NextScene, Scene};

impl NextScene {
    /// Take the next scene initialization function, if it exists
    pub(crate) fn take(&mut self) -> Option<Box<dyn Scene>> {
        self.next_scene.take()
    }
}
