use crate::{Scene, UpdateContext};

impl<'a, Game: crate::Game> UpdateContext<'a, Game> {
    /// Set the next scene to run
    pub fn set_next_scene<S: Scene<Game = Game>>(&mut self, next_scene: S) {
        self.next_scene = Some(Box::new(next_scene));
    }
}
