use crate::render::job::Swapchain;

impl<'surface> Drop for Swapchain<'surface> {
    fn drop(&mut self) {
        self.device.wait_idle().ok();
    }
}
