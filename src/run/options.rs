use argparse::{Command, FlagGroup};

/// Options for controlling how Colosseum runs
#[derive(FlagGroup)]
pub struct ColosseumOptions {}

/// A set of options which contains colosseum options
pub trait GetColosseumOptions: Command {
    /// Get the colosseum options
    fn colosseum_options(&self) -> &ColosseumOptions;
}
