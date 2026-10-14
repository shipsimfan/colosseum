use crate::{
    Result,
    render::{GraphicsDevice, graphics_device::VulkanAdapterInfo},
};
use alexandria::gpu::{VulkanInstance, VulkanSurface};
use colosseum_core::{Logger, new_error};

impl GraphicsDevice {
    /// Get a list of available compatible graphics adapters
    pub(in crate::render::graphics_device) fn get_adapters<'instance>(
        instance: &'instance VulkanInstance,
        surface: &VulkanSurface,
        logger: &Logger,
    ) -> Result<Vec<VulkanAdapterInfo<'instance>>> {
        let adapters = instance
            .enumerate_adapters()
            .map_err(|error| new_error!("unable to enumerate adapters - {}", error))?;

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
