use crate::logging::Logger;
#[cfg(debug_assertions)]
use alexandria::gpu::VulkanDebugMessenger;
use alexandria::{AlexandriaContext, EventPump};
#[cfg(debug_assertions)]
use new::VulkanDebugCallbacks;
use std::sync::{Arc, mpsc::Sender};
use window::SharedWindow;

mod window;

mod get;
mod new;
mod pump;

pub(crate) use window::*;

/// The components needed to run the WSI thread
pub(in crate::run) struct Wsi {
    /// The logger for WSI operations
    logger: Logger,

    /// The Alexandria context
    context: AlexandriaContext<()>,

    /// The event pump for the Alexandria context
    event_pump: EventPump<()>,

    /// The shared state of the window
    shared_window: Arc<SharedWindow>,

    /// The sender for input events from the WSI thread to the game thread
    input_sender: Sender<InputEvent>,

    /// The debug messenger, if validation layers are available and we're in debug mode
    #[cfg(debug_assertions)]
    _debug_messenger: Option<VulkanDebugMessenger<VulkanDebugCallbacks>>,
}
