use alexandria::Id;

use crate::{
    Result,
    render::{AntiAliasingMode, Skybox},
    settings::{ModifiableSettingsCache, SettingsCache},
    update::{Entity, Scene, UpdateContext},
};

impl<'a, Game: crate::Game> UpdateContext<'a, Game> {
    /// Set whether the game should exit after this update
    pub fn set_should_exit(&mut self, should_exit: bool) {
        self.should_exit = should_exit;
    }

    /// Set the next scene to switch to at the start of the next frame
    pub fn set_next_scene<
        S: Scene<Game = Game>,
        F: 'static + FnOnce(&mut UpdateContext<Game>) -> Result<S>,
    >(
        &mut self,
        next_scene: F,
    ) {
        self.next_scene = Some(Box::new(move |context| {
            let scene = next_scene(context)?;
            Ok(Box::new(scene) as _)
        }));
    }

    /// Set the render scale to use for this update
    pub fn set_render_scale(&mut self, render_scale: f32) {
        let mut settings = self.settings.begin_modify();
        settings
            .display_settings_mut()
            .set_render_scale(render_scale);
        self.settings.save(&settings);
    }

    /// Set the gamma to use for this update
    pub fn set_gamma(&mut self, gamma: f32) {
        let mut settings = self.settings.begin_modify();
        settings.display_settings_mut().set_gamma(gamma);
        self.settings.save(&settings);
    }

    /// Set the exposure to use for this update
    pub fn set_exposure(&mut self, exposure: f32) {
        *self.exposure = exposure;
    }

    /// Set the contrast to use for this update
    pub fn set_contrast(&mut self, contrast: f32) {
        *self.contrast = contrast;
    }

    /// Set the saturation to use for this update
    pub fn set_saturation(&mut self, saturation: f32) {
        *self.saturation = saturation;
    }

    /// Set the anti-aliasing mode to use for this update
    pub fn set_anti_aliasing(&mut self, anti_aliasing: AntiAliasingMode) {
        let mut settings = self.settings.begin_modify();
        settings
            .display_settings_mut()
            .set_anti_aliasing(anti_aliasing);
        self.settings.save(&settings);
    }

    /// Set the skybox to use for this update
    pub fn set_skybox<S: Into<Skybox>>(&mut self, skybox: S) {
        *self.skybox = skybox.into();
    }

    /// Set the currently active camera
    pub fn set_active_camera(&mut self, camera: Id<Entity>) {
        *self.active_camera = Some(camera);
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
