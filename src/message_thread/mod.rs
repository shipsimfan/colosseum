use crate::{input::InputDeviceId, logging::Logger};
use events::RawInputButtonEvent;
use shared_state::MessageThreadSharedState;
use std::{
    sync::{Arc, mpsc::Receiver},
    thread::JoinHandle,
};
use thread::message_thread;
use win32::{DWORD, HANDLE, UINT, WM_APP};
use window::Window;

mod events;
mod shared_state;
mod thread;
mod window;

mod drop;
mod get;
mod new;
mod process_inputs;
mod set;

/// A reference to the message pump thread
pub(crate) struct MessageThread {
    /// A logger for printing messages
    logger: Logger,

    /// The shared state of the message thread
    shared_state: Arc<MessageThreadSharedState>,

    /// Mapping of input handles to input device IDs
    handle_to_device_id_map: Vec<(HANDLE, InputDeviceId)>,

    /// A reciever for button input events
    button_events: Receiver<RawInputButtonEvent>,

    /// The id of the message thread
    thread_id: DWORD,

    /// The handle to the thread
    join_handle: Option<JoinHandle<()>>,
}

const WM_APP_SET_DISPLAY_MODE: UINT = WM_APP + 0;
const WM_APP_SET_TITLE: UINT = WM_APP + 1;

/// The maximum number of buttons events to process per frame
const MAX_BUTTON_EVENTS: usize = 64;
