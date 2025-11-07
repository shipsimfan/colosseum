/// A scene which can be used as the starting scene for a game
pub trait InitialScene {
    /// The command line options
    type Options;

    /// Create the new scene using `options`
    fn new(options: &Self::Options) -> Self;
}
