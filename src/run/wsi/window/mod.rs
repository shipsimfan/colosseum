use std::sync::Arc;

mod shared;

mod get;
mod new;

pub(in crate::run::wsi) use shared::SharedWindow;

/// Access to the WSI from the game thread
pub(crate) struct Window {
    /// The shared state of the window
    shared: Arc<SharedWindow>,
}
