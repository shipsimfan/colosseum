use crate::{Error, Result, info, logging::Logger, run::game::GraphicsDevice, warning};
use alexandria::gpu::{VulkanAdapter, VulkanInstance, VulkanSurface};

impl GraphicsDevice {
    /// Get a list of available compatible graphics adapters
    pub(in crate::run::game::graphics_device) fn get_adapters<'instance>(
        instance: &'instance VulkanInstance,
        surface: &VulkanSurface,
        logger: Option<&Logger>,
    ) -> Result<Vec<VulkanAdapter<'instance>>> {
        let adapters = instance
            .enumerate_adapters()
            .map_err(|error| Error::new_inner(error))?;

        Ok(adapters
            .into_iter()
            .filter(|adapter| is_compatible_adapter(adapter, surface, logger))
            .collect())
    }
}

/// Is adapter compatible with the surface and suitable for rendering?
fn is_compatible_adapter(
    adapter: &VulkanAdapter,
    surface: &VulkanSurface,
    logger: Option<&Logger>,
) -> bool {
    if let Some(logger) = logger {
        info!(
            logger,
            "Found adapter: {} ({})",
            adapter.name(),
            adapter.uuid()
        );
    }

    if let Some(logger) = logger {
        warning!(
            logger,
            "Adapter \"{}\" rejected because all are being rejected",
            adapter.name()
        );
    }

    false
}
