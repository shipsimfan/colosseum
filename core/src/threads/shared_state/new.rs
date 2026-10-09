use crate::{GlobalSharedState, logger};
use std::sync::{Mutex, atomic::AtomicBool};

impl<UserEvent: 'static + Send> GlobalSharedState<UserEvent> {
    pub(in crate::threads) fn new() -> GlobalSharedState<UserEvent> {
        GlobalSharedState {
            is_running: AtomicBool::new(true),
            logger: logger!("threads"),
            event_queue: Mutex::new(None),
        }
    }
}
