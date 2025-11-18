use crate::graphics::context::DXGIInfoQueue;

impl Drop for DXGIInfoQueue {
    fn drop(&mut self) {
        self.empty_queue().unwrap();
    }
}
