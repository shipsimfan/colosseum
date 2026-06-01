use crate::{InputEvent, Window, run::wsi::SharedWindow};
use std::sync::{Arc, mpsc::Receiver};

impl Window {
    /// Create a new [`Window`] access structure
    pub(in crate::run::wsi) fn new(
        shared: Arc<SharedWindow>,
        inputs: Receiver<InputEvent>,
    ) -> Window {
        Window { shared, inputs }
    }
}
