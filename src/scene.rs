use crate::{Game, Result, UpdateContext, math::Color3f};

/// A single scene of a game
pub trait Scene: 'static {
    /// The game the scene is for
    type Game: Game;

    /// Called each frame of the game to update the state
    fn update(&mut self, context: &mut UpdateContext<Self::Game>) -> Result<()>;

    /// Get the color to clear the screen with
    fn clear_color(&self) -> Color3f {
        Color3f::BLACK
    }

    /// Get the initial color and intensity of light for the scene
    fn init_ambient(&self) -> (Color3f, f32) {
        (Color3f::WHITE, 0.1)
    }

    /// Called when the scene is set active
    fn on_active(&mut self) {}

    /// Called when the scene is deactivated
    fn on_deactivate(&mut self) {}
}
