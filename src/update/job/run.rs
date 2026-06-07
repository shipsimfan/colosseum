use crate::{
    InputEvent, Result, Window, debug,
    render::RenderData,
    settings::{ModifiableSettingsCache, SettingsCache},
    update::{UpdateContext, UpdateJob},
};
use alexandria::math::Vector2u;
use std::time::Duration;

impl<'a, Game: crate::Game> UpdateJob<'a, Game> {
    /// Run the update job, returning whether the game should continue running after this update
    pub(crate) fn run(
        &mut self,
        window_size: Vector2u,
        delta_time: Duration,
        render_data: &mut RenderData,
        window: &Window,
    ) -> Result<bool> {
        // Update the settings if the window size has changed
        if !self.settings.is_saving() {
            let window_fullscreen = window.fullscreen();
            if window_fullscreen {
                if !self.settings.display_settings().fullscreen() {
                    let mut new_settings = self.settings.begin_modify();
                    new_settings.display_settings_mut().set_fullscreen(true);
                    self.settings.save(&new_settings);
                }
            } else {
                let window_maximized = window.maximized();
                if window_maximized {
                    if !self.settings.display_settings().maximized()
                        || self.settings.display_settings().fullscreen()
                    {
                        let mut new_settings = self.settings.begin_modify();
                        new_settings.display_settings_mut().set_maximized(true);
                        new_settings
                            .display_settings_mut()
                            .set_fullscreen(window_fullscreen);
                        self.settings.save(&new_settings);
                    }
                } else {
                    let window_position = window.position();

                    if self.settings.display_settings().position() != Some(window_position)
                        || self.settings.display_settings().resolution() != Some(window_size)
                        || self.settings.display_settings().maximized()
                        || self.settings.display_settings().fullscreen()
                    {
                        let mut new_settings = self.settings.begin_modify();
                        let display_settings = new_settings.display_settings_mut();

                        display_settings.set_position(window_position);
                        display_settings.set_resolution(window_size);
                        display_settings.set_fullscreen(window_fullscreen);
                        display_settings.set_maximized(window_maximized);

                        self.settings.save(&new_settings);
                    }
                }
            }
        }

        // Process input events
        self.inputs.reset();
        while let Some(input) = window.next_input() {
            match input {
                InputEvent::KeyDown { key } => {
                    self.inputs.set_key_down(key);
                }
                InputEvent::KeyUp { key } => {
                    self.inputs.set_key_up(key);
                }
            }
        }

        // Create the update context for this frame
        let mut update_context = UpdateContext::new(
            delta_time,
            window_size,
            &self.logger,
            self.settings,
            render_data,
            &self.inputs,
            &self.file_io,
            window,
        );

        // Handle any pending scene changes before updating the current scene
        while let Some(mut next_scene) = self.next_scene.take() {
            // Deactivate the current scene
            self.scene.on_deactivate(&mut update_context);

            // Check if the game should exit or if a new scene was set during deactivation
            if update_context.should_exit() {
                return Ok(false);
            }
            if let Some(scene) = update_context.take_next_scene() {
                next_scene = scene;
            }

            // Switch to the next scene and activate it
            debug!(self.logger, "changing scene");
            self.scene = next_scene;
            update_context.render_data().scene_reset();
            self.scene.on_active(&mut update_context);

            // Check if the game should exit or if a new scene was set during activation
            if update_context.should_exit() {
                return Ok(false);
            }
            if let Some(scene) = update_context.take_next_scene() {
                self.next_scene = Some(scene);
            }
        }

        // Update the current scene
        update_context.render_data().reset();
        self.scene.update(&mut update_context)?;

        // Check if the game should exit or if a new scene was set during the update
        if update_context.should_exit() {
            return Ok(false);
        }
        if let Some(next_scene) = update_context.take_next_scene() {
            self.next_scene = Some(next_scene);
        }

        Ok(true)
    }
}
