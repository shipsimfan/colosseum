use crate::message_thread::MessageThreadSharedState;
use std::sync::atomic::Ordering;

impl MessageThreadSharedState {
    /// Set if the window is focused
    pub fn set_is_focused(&self, is_focused: bool) {
        self.is_focused.store(is_focused, Ordering::Release);
    }
}
