use crate::{Result, threads::thread::ThreadSharedState};
use std::{ptr::null_mut, sync::atomic::Ordering};

impl ThreadSharedState {
    /// Get the name of the thread
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Is the thread still running?
    pub fn is_running(&self) -> bool {
        self.is_running.load(Ordering::Acquire)
    }

    /// Get the result of the thread, if it has finished
    pub fn get_result(&self) -> Option<Box<Result<()>>> {
        if self.is_running() {
            None
        } else {
            let result = self.result.swap(null_mut(), Ordering::Acquire);
            if result.is_null() {
                None
            } else {
                Some(unsafe { Box::from_raw(result) })
            }
        }
    }
}
