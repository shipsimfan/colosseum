use crate::{logging::LoggingOptions, settings::SettingsPath};
use argparse::{Command, FlagGroup};

/// Options for controlling how Colosseum runs
#[derive(FlagGroup)]
pub struct ColosseumOptions<Game: crate::Game> {
    /// The folder containing the settings
    #[flag(value = "PATH", default = SettingsPath::default())]
    pub(crate) settings_path: SettingsPath<Game>,

    /// The options to control logging
    #[flag_group]
    pub(crate) logging_options: LoggingOptions<Game>,
}

/// A set of options which contains colosseum options
pub trait GameOptions<Game: crate::Game>: Command + Send {
    /// Get the colosseum options
    fn colosseum_options(&self) -> &ColosseumOptions<Game>;
}
