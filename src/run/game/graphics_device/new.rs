use crate::{Error, Result, logging::Logger, run::game::GraphicsDevice};
use alexandria::gpu::{VulkanInstance, VulkanSurface};

impl GraphicsDevice {
    /// Locate a new graphics device based on the name or UUID of the adapter, or use a default if none is provided
    pub fn new(
        adapter: Option<&str>,
        instance: &VulkanInstance,
        surface: &VulkanSurface,
        logger: &Logger,
    ) -> Result<Self> {
        let logger = logger.logger("vulkan");

        let adapters = GraphicsDevice::get_adapters(instance, surface, Some(&logger))?;
        if adapters.len() == 0 {
            return Err(Error::new("no compatible graphics adapters found"));
        }

        todo!()
    }
}
