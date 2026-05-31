use crate::{Window, run::wsi::SharedWindow};
use std::sync::Arc;

impl Window {
    /// Create a new [`Window`] access structure
    pub(in crate::run::wsi) fn new(shared: Arc<SharedWindow>) -> Window {
        Window { shared }
    }
}
