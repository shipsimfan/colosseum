use crate::{
    Error, Result,
    logging::Logger,
    render::job::{GraphicsDevice, graphics_device::VulkanAdapterInfo},
};
use alexandria::gpu::{VulkanInstance, VulkanSurface};

impl<'surface> GraphicsDevice<'surface> {
    /// Get a list of available compatible graphics adapters
    pub(in crate::render::job::graphics_device) fn get_adapters<'instance>(
        instance: &'instance VulkanInstance,
        surface: &'surface VulkanSurface,
        logger: Option<&Logger>,
    ) -> Result<Vec<VulkanAdapterInfo<'instance>>> {
        let adapters = instance.enumerate_adapters().map_err(Error::new_inner)?;

        let mut adapters: Vec<_> = adapters
            .into_iter()
            .filter_map(|adapter| {
                VulkanAdapterInfo::is_compatible_adapter(adapter, surface, logger).transpose()
            })
            .collect::<Result<_>>()?;
        adapters.sort();
        Ok(adapters)
    }
}
