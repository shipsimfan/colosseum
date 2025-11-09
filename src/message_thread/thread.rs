use crate::{
    Result, RunningState, debug,
    graphics::GraphicsSettings,
    logging::Logger,
    message_thread::{MessageThreadSharedState, RawInputButtonEvent, Window},
    util::message_box,
};
use std::sync::{Arc, mpsc::SyncSender};
use win32::{DWORD, GetCurrentThreadId};

/// The main function for the message thread
///
/// This function creates a window, then returns the handle to the window and this thread's ID in
/// `initialization_data` so it can be used for creating graphics objects and for killing this
/// thread.
pub(in crate::message_thread) fn message_thread(
    title: &str,
    settings: GraphicsSettings,
    shared_state: Arc<MessageThreadSharedState>,
    button_event_queue: SyncSender<RawInputButtonEvent>,
    logger: Logger,
    initialization_data: SyncSender<Result<(usize, DWORD)>>,
    running_state: Arc<RunningState>,
) {
    // Create window
    let mut window = match init(
        title,
        settings,
        running_state.clone(),
        shared_state,
        button_event_queue,
        logger.clone(),
    ) {
        Ok((window, thread_id)) => {
            initialization_data
                .send(Ok((window.handle() as _, thread_id)))
                .unwrap();
            window
        }
        Err(error) => {
            initialization_data.send(Err(error)).unwrap();
            return;
        }
    };
    drop(initialization_data);

    // Main message loop
    while window.is_running() {
        if let Err(error) = window.process_messages() {
            let error = format!("failed to process messages - {}", error);
            crate::error!(logger, "{}", error);
            message_box("Error", &error, Some(window.handle())).ok();
            break;
        }
    }

    debug!(logger, "Exiting message thread");

    // Signal thread exit
    running_state.kill();
}

fn init(
    title: &str,
    settings: GraphicsSettings,
    running_state: Arc<RunningState>,
    shared_state: Arc<MessageThreadSharedState>,
    button_event_queue: SyncSender<RawInputButtonEvent>,
    logger: Logger,
) -> Result<(Box<Window>, DWORD)> {
    // Change title to UTF-16
    let mut title: Vec<_> = title.encode_utf16().collect();
    title.push(0);

    // Create window
    let window = Window::new(
        &title,
        settings.x,
        settings.y,
        settings.width,
        settings.height,
        settings.display_mode,
        running_state,
        shared_state,
        button_event_queue,
        logger,
    )?;

    // Get thread ID
    let thread_id = unsafe { GetCurrentThreadId() };

    Ok((window, thread_id))
}
