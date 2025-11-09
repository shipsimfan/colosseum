use crate::{
    Error, MessageThread, Result, RunningState,
    graphics::GraphicsSettings,
    input::InputDeviceId,
    logging::LogController,
    message_thread::{MAX_BUTTON_EVENTS, message_thread, shared_state::MessageThreadSharedState},
};
use std::{ptr::null_mut, sync::Arc};
use win32::HWND;

impl MessageThread {
    /// Launch a new [`MessageThread`]
    pub fn new(
        title: &'static str,
        settings: GraphicsSettings,
        log_controller: &Arc<LogController>,
        keyboard_id: InputDeviceId,
        running_state: Arc<RunningState>,
    ) -> Result<(Self, HWND)> {
        // Create logger
        let logger = log_controller.logger("window");

        // Create shared state
        let shared_state = MessageThreadSharedState::new();
        let (initialization_data_sender, initialization_data) = std::sync::mpsc::sync_channel(1);
        let (button_event_queue, button_events) = std::sync::mpsc::sync_channel(MAX_BUTTON_EVENTS);

        // Spawn thread
        let child_shared_state = shared_state.clone();
        let child_logger = logger.clone();
        let join_handle = Some(
            std::thread::Builder::new()
                .name("Message Pump".to_string())
                .spawn(move || {
                    message_thread(
                        title,
                        settings,
                        child_shared_state,
                        button_event_queue,
                        child_logger,
                        initialization_data_sender,
                        running_state,
                    )
                })
                .map_err(|error| Error::new_inner("unable to spawn message thread", error))?,
        );

        // Wait for initialization data
        let (hwnd, thread_id) = initialization_data.recv().unwrap()?;
        let hwnd = hwnd as HWND;

        // Create initial device map
        let handle_to_device_id_map = vec![(null_mut(), keyboard_id)];

        Ok((
            MessageThread {
                logger,
                shared_state,
                handle_to_device_id_map,
                button_events,
                thread_id,
                join_handle,
            },
            hwnd,
        ))
    }
}
