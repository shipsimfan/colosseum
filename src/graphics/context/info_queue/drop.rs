use crate::graphics::context::InfoQueue;

impl Drop for InfoQueue {
    fn drop(&mut self) {
        self.empty_queue().unwrap();
    }
}
