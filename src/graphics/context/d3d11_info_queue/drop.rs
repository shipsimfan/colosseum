use crate::graphics::context::D3D11InfoQueue;

impl Drop for D3D11InfoQueue {
    fn drop(&mut self) {
        self.empty_queue().unwrap();
    }
}
