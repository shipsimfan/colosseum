use crate::{
    Result, RunningState, debug,
    graphics::DisplayMode,
    logging::Logger,
    message_thread::{
        MessageThreadSharedState, Window,
        window::{WindowClass, WindowHandle},
    },
};
use std::sync::Arc;

impl Window {
    /// Create a new [`Window`] for rendering
    pub fn new(
        title: &[u16],
        x: Option<i32>,
        y: Option<i32>,
        width: Option<u32>,
        height: Option<u32>,
        display_mode: DisplayMode,
        shared_state: Arc<MessageThreadSharedState>,
        running_state: Arc<RunningState>,
        logger: Logger,
    ) -> Result<Box<Self>> {
        assert!(title.last().is_some());
        assert_eq!(*title.last().unwrap(), 0);

        // Create window class
        let class = WindowClass::register(&title)?;

        // Create window
        let mut window = Box::new_uninit();
        let handle = WindowHandle::create(
            &title,
            &class,
            x,
            y,
            width,
            height,
            display_mode,
            window.as_mut_ptr(),
        )?;

        // Get position and size
        let (position, size) = handle.get_size_and_position()?;
        debug!(
            logger,
            "Window sized {}x{} created at {} (display mode: {})",
            size.x,
            size.y,
            position,
            display_mode
        );

        // Write info into output box
        Ok(Box::write(
            window,
            Window {
                logger,
                running_state,
                shared_state,
                is_running: true,
                position,
                size,
                is_focused: true,
                in_move: false,
                wnd_proc_result: Ok(()),
                handle,
                class,
            },
        ))
    }
}
