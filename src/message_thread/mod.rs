use shared_state::MessageThreadSharedState;
use std::{sync::Arc, thread::JoinHandle};
use thread::message_thread;
use win32::{DWORD, UINT, WM_APP};
use window::Window;

mod shared_state;
mod thread;
mod window;

mod drop;
mod get;
mod new;
mod set;

/// A reference to the message pump thread
pub(crate) struct MessageThread {
    /// The shared state of the message thread
    shared_state: Arc<MessageThreadSharedState>,

    /// The id of the message thread
    thread_id: DWORD,

    /// The handle to the thread
    join_handle: Option<JoinHandle<()>>,
}

const WM_APP_SET_DISPLAY_MODE: UINT = WM_APP + 0;
const WM_APP_SET_TITLE: UINT = WM_APP + 1;
