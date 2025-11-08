use crate::logging::LoggingOptions;
use argparse::{Command, FlagGroup};

/// Options for controlling how Colosseum runs
#[derive(FlagGroup)]
pub struct ColosseumOptions<Game: crate::Game> {
    /// The options to control logging
    #[flag_group]
    pub(crate) logging_options: LoggingOptions<Game>,
}

/// A set of options which contains colosseum options
pub trait GetColosseumOptions<Game: crate::Game>: Command {
    /// Get the colosseum options
    fn colosseum_options(&self) -> &ColosseumOptions<Game>;
}
