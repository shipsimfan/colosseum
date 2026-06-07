use crate::SingleValueReceiver;
use std::sync::atomic::Ordering;

impl<T> SingleValueReceiver<T> {
    /// Is the value available to be taken?
    pub fn is_available(&self) -> bool {
        self.shared_state.sent.load(Ordering::Acquire)
    }
}
