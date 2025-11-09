use crate::{
    Result, RunningState,
    logging::Logger,
    math::{Vector2i, Vector2u},
    message_thread::{MessageThreadSharedState, RawInputButtonEvent},
};
use std::sync::{Arc, mpsc::SyncSender};
use window_class::WindowClass;
use window_handle::WindowHandle;

mod window_class;
mod window_handle;

mod drop;
mod get;
mod new;
mod process_messages;
mod window_proc;

/// A window which can be rendered into and receive input
pub struct Window {
    /// The logger to use for events
    logger: Logger,

    /// Tracks if the engine is currently running
    running_state: Arc<RunningState>,

    /// The shared state to report changes into
    shared_state: Arc<MessageThreadSharedState>,

    /// The queue to send button events on
    button_event_queue: SyncSender<RawInputButtonEvent>,

    /// Is the window still running?
    is_running: bool,

    /// The position of the upper-left corner of the client area of the window
    position: Vector2i,

    /// The size of the client area of the window
    size: Vector2u,

    /// Is this window the one being focused on?
    is_focused: bool,

    /// Is the window being actively moved or resized?
    in_move: bool,

    /// The result of the window procedure
    wnd_proc_result: Result<()>,

    /// The handle to the window itself
    handle: WindowHandle,

    /// The class the window belongs to
    #[allow(unused)]
    class: WindowClass,
}
