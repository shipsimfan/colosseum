use crate::{Scene, UpdateContext};

impl<'a, Game: crate::Game> UpdateContext<'a, Game> {
    /// Set the next scene to run
    pub fn set_next_scene<
        S: Scene<Game = Game>,
        F: 'static + FnOnce(&mut UpdateContext<Game>) -> S,
    >(
        &mut self,
        next_scene: F,
    ) {
        self.next_scene = Some(Box::new(|context| Box::new(next_scene(context))));
    }
}
