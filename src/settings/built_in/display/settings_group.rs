use crate::settings::{DisplaySettings, SettingsGroup};

impl SettingsGroup for DisplaySettings {
    const FILE_NAME: &str = "display";
    const PRETTY_NAME: &str = "Display";
}
