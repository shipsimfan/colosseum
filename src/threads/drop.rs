use crate::ThreadManager;

impl Drop for ThreadManager {
    fn drop(&mut self) {
        self.kill().unwrap();
    }
}
