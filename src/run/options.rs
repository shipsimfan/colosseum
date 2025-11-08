use crate::{logging::LoggingOptions, settings::SettingsPath};
use argparse::{Command, FlagGroup};
use std::marker::PhantomData;

/// Options for controlling how Colosseum runs
#[derive(FlagGroup)]
pub struct ColosseumOptions<Game: crate::Game> {
    /// The folder containing the settings
    #[flag(value = "PATH", default = SettingsPath::Default(PhantomData))]
    pub(crate) settings_path: SettingsPath<Game>,

    /// The options to control logging
    #[flag_group]
    pub(crate) logging_options: LoggingOptions<Game>,
}

/// A set of options which contains colosseum options
pub trait GetColosseumOptions<Game: crate::Game>: Command {
    /// Get the colosseum options
    fn colosseum_options(&self) -> &ColosseumOptions<Game>;
}
