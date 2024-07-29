use super::Colosseum;

/// The context which scenes use to render
pub struct RenderContext<'a>(&'a mut Colosseum);

impl<'a> RenderContext<'a> {
    /// Creates a new [`RenderContext`]
    pub(super) fn new(colosseum: &'a mut Colosseum) -> Self {
        RenderContext(colosseum)
    }
}
