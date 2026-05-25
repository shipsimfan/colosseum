use crate::threads::thread::ThreadSharedState;
use std::{
    ptr::null_mut,
    sync::atomic::{AtomicBool, AtomicPtr},
};

impl ThreadSharedState {
    /// Create a new [`ThreadSharedState`]
    pub fn new(name: String) -> ThreadSharedState {
        ThreadSharedState {
            name,
            is_running: AtomicBool::new(true),
            result: AtomicPtr::new(null_mut()),
        }
    }
}
