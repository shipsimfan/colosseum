use crate::{NextSceneFn, UpdateContext};

impl<'a, Game: crate::Game> UpdateContext<'a, Game> {
    /// Get the amount of time that has passed since the last frame, in seconds
    pub const fn delta_t(&self) -> f32 {
        self.delta_t
    }

    /// Take the next scene from this context, if one was set
    pub(crate) fn take_next_scene(&mut self) -> Option<NextSceneFn<Game>> {
        self.next_scene.take()
    }
}
