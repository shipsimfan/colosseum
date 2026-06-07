use crate::{
    Error, InputEvent, Result, UserEvent, debug,
    logging::Logger,
    run::{Wsi, wsi::SharedWindow},
    settings::DisplaySettings,
};
use alexandria::{
    AlexandriaContext,
    gpu::{VulkanInstance, VulkanSurface, VulkanVersion},
};
use std::{
    str::FromStr,
    sync::{
        Arc,
        mpsc::{self, Receiver},
    },
};

#[cfg(debug_assertions)]
mod debug_messenger;
mod vulkan_instance;

#[cfg(debug_assertions)]
pub(in crate::run::wsi) use debug_messenger::VulkanDebugCallbacks;

impl Wsi {
    /// Creates a new [`Wsi`]
    pub fn new(
        game_name: &str,
        game_version: &str,
        logger: &Logger,
        display_settings: &DisplaySettings,
    ) -> Result<(Wsi, VulkanInstance, VulkanSurface, Receiver<InputEvent>)> {
        // Create the Alexandria context
        let (context, event_pump) = AlexandriaContext::<UserEvent>::builder()
            .gpu()
            .window()
            .create()
            .map_err(Error::new_inner)?;
        debug!(logger, "Created Alexandria context");

        // Create window
        let mut builder = context
            .window()
            .create_window(format!("{} v{}", game_name, game_version));
        builder
            .position(display_settings.position())
            .size(display_settings.resolution())
            .resizable()
            .bordered();
        if display_settings.fullscreen() {
            builder.fullscreen();
        }
        if display_settings.maximized() {
            builder.maximized();
        }

        let window = builder.create().map_err(Error::new_inner)?;

        // Create the shared window state
        let shared_window = Arc::new(SharedWindow::new(
            window.position(),
            window.size(),
            window.is_fullscreen(),
            window.is_maximized(),
        )?);

        // Create the Vulkan instance and check for validation layers
        let vulkan_logger = logger.logger("vulkan");
        #[cfg_attr(not(debug_assertions), allow(unused_variables))]
        let (vulkan_instance, create_debug_messenger) = vulkan_instance::create(
            &context.gpu(),
            &vulkan_logger,
            game_name,
            VulkanVersion::from_str(game_version)
                .map_err(|_| Error::new("invalid game version"))?,
            &window,
        )?;

        // Create the debug messenger if validation layers are available and we're in debug mode
        #[cfg(debug_assertions)]
        let _debug_messenger = if create_debug_messenger {
            Some(debug_messenger::create(&vulkan_instance, &vulkan_logger)?)
        } else {
            None
        };

        // Create the window surface
        let surface = vulkan_instance
            .create_window_surface(&window)
            .map_err(Error::new_inner)?;

        // Create the input event channel
        let (input_sender, input_receiver) = mpsc::channel();

        Ok((
            Wsi {
                logger: logger.logger("wsi"),
                context,
                event_pump,
                window,
                shared_window,
                input_sender,
                #[cfg(debug_assertions)]
                _debug_messenger,
            },
            vulkan_instance,
            surface,
            input_receiver,
        ))
    }
}
