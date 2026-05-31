use crate::update::{Scene, UpdateContext};
use alexandria::math::{Color3f, Linear};

impl<'a, Game: crate::Game> UpdateContext<'a, Game> {
    /// Set whether the game should exit after this update
    pub fn set_should_exit(&mut self, should_exit: bool) {
        self.should_exit = should_exit;
    }

    /// Set the next scene to switch to at the start of the next frame
    pub fn set_next_scene(&mut self, next_scene: Box<dyn Scene<Game = Game>>) {
        self.next_scene = Some(next_scene);
    }

    /// Set the color to clear the screen with before rendering
    pub fn set_clear_color(&mut self, clear_color: Color3f<Linear>) {
        self.render_data.set_clear_color(clear_color);
    }
}
