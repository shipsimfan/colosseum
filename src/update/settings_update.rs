use crate::{ModifiableSettingsCache, SettingsCache, Window};
use colosseum_ecs::{Res, ResMut};

/// Update the settings based on the current state of the window
pub(in crate::update) fn settings_update_system<Game: crate::Game>(
    window: Res<Window>,
    mut settings: ResMut<Game::SettingsCache>,
) {
    // Update the settings if the window size has changed
    if settings.is_saving() {
        return;
    }

    let display_settings = settings.display_settings();

    let window_fullscreen = window.fullscreen();
    if window_fullscreen {
        if !display_settings.fullscreen() {
            let mut new_settings = settings.begin_modify();
            new_settings.display_settings_mut().set_fullscreen(true);
            settings.save(&new_settings);
        }

        return;
    }

    let window_maximized = window.maximized();
    if window_maximized {
        if !display_settings.maximized() || display_settings.fullscreen() {
            let mut new_settings = settings.begin_modify();
            new_settings.display_settings_mut().set_maximized(true);
            new_settings
                .display_settings_mut()
                .set_fullscreen(window_fullscreen);
            settings.save(&new_settings);
        }

        return;
    }

    let window_position = window.position();
    let window_size = window.size();

    if display_settings.position() != Some(window_position)
        || display_settings.resolution() != Some(window_size)
        || display_settings.maximized()
        || display_settings.fullscreen()
    {
        let mut new_settings = settings.begin_modify();
        let display_settings = new_settings.display_settings_mut();

        display_settings.set_position(window_position);
        display_settings.set_resolution(window_size);
        display_settings.set_fullscreen(window_fullscreen);
        display_settings.set_maximized(window_maximized);

        settings.save(&new_settings);
    }
}
