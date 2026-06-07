use crate::UserEvent;
use alexandria::EventQueue;
use std::sync::{Arc, mpsc::Receiver};

mod input_event;
mod shared;

mod get;
mod new;
mod set;

pub(crate) use input_event::InputEvent;

pub(in crate::run::wsi) use shared::SharedWindow;

/// Access to the WSI from the game thread
pub(crate) struct Window {
    /// The shared state of the window
    shared: Arc<SharedWindow>,

    /// The queue of input events
    inputs: Receiver<InputEvent>,

    /// A queue to submit events to the WSI thread
    event_queue: EventQueue<UserEvent>,
}
