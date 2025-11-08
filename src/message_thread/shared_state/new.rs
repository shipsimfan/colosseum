use crate::message_thread::MessageThreadSharedState;
use std::sync::{
    Arc, Mutex,
    atomic::{AtomicBool, AtomicU64},
};

impl MessageThreadSharedState {
    /// Create a new [`MessageThreadSharedState`]
    pub fn new() -> Arc<Self> {
        Arc::new(MessageThreadSharedState {
            hwnd_valid: Mutex::new(true),
            position: AtomicU64::new(0),
            size: AtomicU64::new(0),
            is_focused: AtomicBool::new(true),
        })
    }
}
