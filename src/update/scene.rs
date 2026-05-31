use crate::{Game, Result, update::UpdateContext};

/// A single scene of a game
pub trait Scene: 'static {
    /// The game which this scene belongs to
    type Game: Game;

    /// Called each frame of the game to update the state
    fn update(&mut self, context: &mut UpdateContext<Self::Game>) -> Result<()>;

    /// Called when the scene is set active
    #[allow(unused_variables)]
    fn on_active(&mut self, context: &mut UpdateContext<Self::Game>) {}

    /// Called when the scene is deactivated
    #[allow(unused_variables)]
    fn on_deactivate(&mut self, context: &mut UpdateContext<Self::Game>) {}
}

/// A scene which can be used as the starting scene for a game
pub trait InitialScene: Scene + Sized {
    /// Create the new scene using `options`
    fn new(
        options: &<Self::Game as Game>::Options,
        context: &mut UpdateContext<Self::Game>,
    ) -> Result<Self>;
}
