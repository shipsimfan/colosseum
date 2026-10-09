use crate::{LogSeverity, Logger, Result, log};
use alexandria::gpu::{
    VulkanDebugMessageSeverityFlag, VulkanDebugMessageTypeFlag, VulkanDebugMessageTypeFlags,
    VulkanDebugMessenger, VulkanDebugMessengerCallback, VulkanInstance,
};
use colosseum_core::new_error;

/// Create a new [`VulkanDebugMessenger`]
pub(in crate::run::wsi::new) fn create(
    vulkan_instance: &VulkanInstance,
    logger: &Logger,
) -> Result<VulkanDebugMessenger<VulkanDebugCallbacks>> {
    vulkan_instance
        .create_debug_messenger(
            VulkanDebugMessageSeverityFlag::ErrorExt
                | VulkanDebugMessageSeverityFlag::WarningExt
                | VulkanDebugMessageSeverityFlag::InfoExt
                | VulkanDebugMessageSeverityFlag::VerboseExt,
            VulkanDebugMessageTypeFlag::AddressBindingExt
                | VulkanDebugMessageTypeFlag::GeneralExt
                | VulkanDebugMessageTypeFlag::PerformanceExt
                | VulkanDebugMessageTypeFlag::ValidationExt,
            VulkanDebugCallbacks {
                logger: logger.clone(),
            },
        )
        .map_err(|error| new_error!("unable to create Vulkan debug messenger - {}", error))
}

/// The callbacks for the Vulkan debug messenger
pub(in crate::run::wsi) struct VulkanDebugCallbacks {
    /// The logger to use for the debug callbacks
    logger: Logger,
}

impl VulkanDebugMessengerCallback for VulkanDebugCallbacks {
    fn message(
        &self,
        message: &str,
        severity: VulkanDebugMessageSeverityFlag,
        _: VulkanDebugMessageTypeFlags,
    ) {
        let severity = match severity {
            VulkanDebugMessageSeverityFlag::ErrorExt => LogSeverity::Error,
            VulkanDebugMessageSeverityFlag::WarningExt => LogSeverity::Warning,
            VulkanDebugMessageSeverityFlag::InfoExt => LogSeverity::Info,
            _ => LogSeverity::Debug,
        };

        log!(severity, self.logger, "{}", message);
    }
}
