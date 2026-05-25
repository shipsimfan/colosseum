use crate::{Result, threads::thread::ThreadSharedState};
use std::sync::atomic::Ordering;

impl ThreadSharedState {
    /// Signal that the thread has finished and provide the result of the thread's execution
    pub fn kill(&self, result: Result<()>) {
        let result = Box::into_raw(Box::new(result));
        self.result.store(result, Ordering::Release);

        self.is_running.store(false, Ordering::Release);
    }
}
