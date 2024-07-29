use super::Colosseum;
use std::marker::PhantomData;

/// The context which scenes use to render
pub struct RenderContext<'a> {
    _phantom: PhantomData<&'a mut ()>,
}

impl<'a> RenderContext<'a> {
    /// Creates a new [`UpdateContext`]
    pub(super) fn new(colosseum: &'a mut Colosseum) -> Self {
        RenderContext {
            _phantom: PhantomData,
        }
    }
}
