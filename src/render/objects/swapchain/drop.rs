use crate::render::Swapchain;

impl Drop for Swapchain {
    fn drop(&mut self) {
        self.device.wait_idle().ok();
    }
}
