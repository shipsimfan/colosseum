use crate::{
    Result,
    render::Skybox,
    update::{Scene, UpdateContext},
};

impl<'a, Game: crate::Game> UpdateContext<'a, Game> {
    /// Set whether the game should exit after this update
    pub fn set_should_exit(&mut self, should_exit: bool) {
        self.should_exit = should_exit;
    }

    /// Set the next scene to switch to at the start of the next frame
    pub fn set_next_scene(&mut self, next_scene: Box<dyn Scene<Game = Game>>) {
        self.next_scene = Some(next_scene);
    }

    /// Set the skybox to use for this update
    pub fn set_skybox<S: Into<Skybox>>(&mut self, skybox: S) {
        *self.skybox = skybox.into();
    }

    /// Set the window to fullscreen mode
    pub fn set_fullscreen(&self) -> Result<()> {
        self.window.set_fullscreen()
    }

    /// Unset the window from fullscreen mode
    pub fn unset_fullscreen(&self) -> Result<()> {
        self.window.unset_fullscreen()
    }
}
