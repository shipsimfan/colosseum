use crate::logging::Logger;
#[cfg(debug_assertions)]
use alexandria::gpu::VulkanDebugMessenger;
use alexandria::{AlexandriaContext, EventPump, window::Window};
#[cfg(debug_assertions)]
use new::VulkanDebugCallbacks;

mod get;
mod new;
mod pump;

/// The components needed to run the WSI thread
pub(in crate::run) struct Wsi {
    /// The logger for WSI operations
    logger: Logger,

    /// The Alexandria context
    context: AlexandriaContext<()>,

    /// The event pump for the Alexandria context
    event_pump: EventPump<()>,

    /// The window being managed by the WSI
    window: Window<()>,

    /// The debug messenger, if validation layers are available and we're in debug mode
    #[cfg(debug_assertions)]
    _debug_messenger: Option<VulkanDebugMessenger<VulkanDebugCallbacks>>,
}
