use crate::{Result, logging::Logger};
use alexandria::gpu::{GpuSubsystem, VulkanInstanceExtension};

/// Get the Vulkan instance layers and extensions, returning whether the debug messenger should be
/// created
pub(in crate::run::wsi::new::vulkan_instance) fn get_layers_and_extensions(
    _: &GpuSubsystem,
    _: &Logger,
) -> Result<(Vec<String>, Vec<VulkanInstanceExtension>, bool)> {
    Ok((Vec::new(), Vec::new(), false))
}
