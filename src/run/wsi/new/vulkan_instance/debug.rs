use crate::{Error, Result, debug, logging::Logger, warning};
use alexandria::gpu::{GpuSubsystem, VulkanInstanceExtension};

const VALIDATION_LAYER_NAME: &str = "VK_LAYER_KHRONOS_validation";

/// Get the Vulkan instance layers and extensions, returning whether the debug messenger should be
/// created
pub(in crate::run::wsi::new::vulkan_instance) fn get_layers_and_extensions(
    gpu: &GpuSubsystem,
    logger: &Logger,
) -> Result<(Vec<String>, Vec<VulkanInstanceExtension>, bool)> {
    // Check for validation layers
    let mut has_validation_layers = false;
    for layer in gpu.layers().map_err(|error| Error::new_inner(error))? {
        if layer.name() == VALIDATION_LAYER_NAME {
            has_validation_layers = true;
            break;
        }
    }

    // Log whether validation layers are available
    let layers = if has_validation_layers {
        debug!(logger, "Validation layers available");
        vec![VALIDATION_LAYER_NAME.to_string()]
    } else {
        warning!(
            logger,
            "Validation layers not available, debug mode will be limited"
        );
        Vec::new()
    };

    // Check for the debug utils extension
    let mut has_debug_utils_extension = false;
    for extension in gpu
        .extensions(None)
        .map_err(|error| Error::new_inner(error))?
    {
        if extension == VulkanInstanceExtension::DebugUtils {
            has_debug_utils_extension = true;
            break;
        }
    }

    // Log whether the debug utils extension is available
    let extensions = if has_debug_utils_extension {
        debug!(logger, "Debug utils extension available");
        vec![VulkanInstanceExtension::DebugUtils]
    } else {
        warning!(
            logger,
            "Debug utils extension not available, no debug messenger will be created"
        );
        Vec::new()
    };

    Ok((layers, extensions, has_debug_utils_extension))
}
