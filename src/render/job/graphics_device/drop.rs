use crate::render::job::GraphicsDevice;

impl Drop for GraphicsDevice {
    fn drop(&mut self) {
        self.device.wait_idle().ok();
    }
}
