use crate::{GlobalSharedState, logging::LogController};
use std::sync::{Arc, atomic::AtomicBool};

impl GlobalSharedState {
    pub(in crate::threads) fn new(log_controller: &Arc<LogController>) -> GlobalSharedState {
        GlobalSharedState {
            is_running: AtomicBool::new(true),
            logger: log_controller.logger("threads"),
        }
    }
}
