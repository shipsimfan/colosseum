use crate::{Game, UpdateContext};

/// A single scene of a game
pub trait Scene {
    /// The game the scene is for
    type Game: Game;

    /// Called each frame of the game to update the state
    fn update(&mut self, context: &mut UpdateContext<Self::Game>);

    /// Called when the scene is set active
    fn on_active(&mut self) {}

    /// Called when the scene is deactivated
    fn on_deactivate(&mut self) {}
}
