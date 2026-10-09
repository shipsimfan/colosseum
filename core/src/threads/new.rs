use crate::{
    GlobalSharedState, LogController, Result, ThreadManager, logger, single_value_channel,
};
use std::sync::{Arc, Mutex};

impl<UserData: 'static + Send> ThreadManager<UserData> {
    /// Create a new [`ThreadManager`]
    pub fn new() -> Result<Arc<ThreadManager<UserData>>> {
        let shared_state = Arc::new(GlobalSharedState::new());
        let (panic_sender, panic_receiver) = single_value_channel(false)?;

        let child_shared_state = shared_state.clone();
        let panic_sender = Mutex::new(Some(panic_sender));
        std::panic::set_hook(Box::new(move |panic_info| {
            let current_thread = std::thread::current();
            let thread_name = current_thread.name().unwrap_or("unknown");
            eprintln!(
                "[PANIC] Thread \"{}\" panicked: {}",
                thread_name, panic_info
            );
            if let Ok(Some(panic_sender)) = panic_sender.try_lock().map(|mut sender| sender.take())
            {
                panic_sender
                    .send(format!(
                        "Thread \"{}\" panicked: {}",
                        thread_name, panic_info
                    ))
                    .ok();
            }
            child_shared_state.kill(&thread_name);
        }));

        let thread_manager = Arc::new(ThreadManager {
            shared_state,
            threads: Mutex::new(Vec::new()),
            logger: logger!("threads"),
            panic_receiver: Mutex::new(Some(panic_receiver)),
        });
        LogController::get().spawn_thread(&thread_manager)?;

        Ok(thread_manager)
    }
}
