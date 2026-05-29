use crate::render::job::GraphicsDevice;

impl<'surface> Drop for GraphicsDevice<'surface> {
    fn drop(&mut self) {
        self.device.wait_idle().ok();
    }
}
