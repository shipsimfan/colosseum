use crate::UpdateContext;

impl<'a, Game: crate::Game> UpdateContext<'a, Game> {
    /// Quit the game, exiting the application
    pub fn quit(&self) {
        self.running_state.kill();
    }
}
