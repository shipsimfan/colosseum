use crate::threads::thread::ThreadSharedState;
use std::sync::atomic::Ordering;

impl Drop for ThreadSharedState {
    fn drop(&mut self) {
        let result = self.result.load(Ordering::Acquire);
        if !result.is_null() {
            // We need to cleanup the result
            unsafe { drop(Box::from_raw(result)) };
        }
    }
}
