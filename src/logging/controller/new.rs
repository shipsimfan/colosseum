use crate::{
    Result,
    logging::{LogController, LoggingOptions},
};
use std::{
    sync::{Arc, Mutex, atomic::AtomicU64},
    time::Instant,
};

impl LogController {
    /// Creates a new [`LogController`]
    pub(crate) fn new<Game: crate::Game>(options: &LoggingOptions<Game>) -> Result<Arc<Self>> {
        // Get the start time
        let start_time = Instant::now();

        // Create the message queue
        let (message_queue, message_queue_recv) = std::sync::mpsc::channel();

        Ok(Arc::new(LogController {
            minimum_severity: options.min_log_severity,
            frame: AtomicU64::new(0),
            start_time,
            message_queue,
            reciever: Mutex::new(Some(message_queue_recv)),
        }))
    }
}
