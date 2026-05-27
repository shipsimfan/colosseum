use crate::{
    Error, Result, log,
    logging::{LogSeverity, Logger},
};
use alexandria::gpu::{
    VulkanDebugMessageSeverity, VulkanDebugMessenger, VulkanDebugMessengerCallback, VulkanInstance,
};

/// Create a new [`VulkanDebugMessenger`]
pub(in crate::run::wsi::new) fn create(
    vulkan_instance: &VulkanInstance,
    logger: &Logger,
) -> Result<VulkanDebugMessenger<VulkanDebugCallbacks>> {
    vulkan_instance
        .create_debug_messenger(
            VulkanDebugMessageSeverity::Verbose,
            VulkanDebugCallbacks {
                logger: logger.clone(),
            },
        )
        .map_err(|error| Error::new_inner(error))
}

/// The callbacks for the Vulkan debug messenger
pub(in crate::run::wsi) struct VulkanDebugCallbacks {
    /// The logger to use for the debug callbacks
    logger: Logger,
}

impl VulkanDebugMessengerCallback for VulkanDebugCallbacks {
    fn message(&self, message: &str, severity: VulkanDebugMessageSeverity) {
        let severity = match severity {
            VulkanDebugMessageSeverity::Error => LogSeverity::Error,
            VulkanDebugMessageSeverity::Warning => LogSeverity::Warning,
            VulkanDebugMessageSeverity::Info => LogSeverity::Info,
            VulkanDebugMessageSeverity::Verbose => LogSeverity::Debug,
        };

        log!(severity, self.logger, "{}", message);
    }
}
