/// Metadata about a game
pub trait GameMetadata: 'static {
    /// The name of the game, to be used as the title for the window
    const NAME: &str;

    /// The name of the company making the game, used to automatically produce appropriate folders
    const COMPANY: &str;

    /// The version of the game
    ///
    /// Typically you should set this to `env!("CARGO_PKG_VERSION")`
    const VERSION: &str;
}
