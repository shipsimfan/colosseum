use crate::{
    Error, GlobalSharedState, Result, debug, error,
    threads::{Thread, single_value_channel},
};
use std::sync::{Arc, Mutex};

impl Thread {
    /// Create a new thread with the provided name
    pub fn new<
        F1: 'static + FnOnce(&GlobalSharedState) -> Result<()> + Send,
        F2: 'static + FnOnce() + Send,
    >(
        name: String,
        global_shared_state: Arc<GlobalSharedState>,
        f: F1,
        on_kill: F2,
    ) -> Result<Thread> {
        let (result_sender, result_receiver) = single_value_channel::create(true)?;

        let child_name = name.clone();
        let join_handle = std::thread::Builder::new()
            .name(name.clone())
            .spawn(move || {
                let result_sender = Arc::new(Mutex::new(Some(result_sender)));

                // Setup a panic hook to kill the thread if it panics
                let child_shared_state = global_shared_state.clone();
                let child_result_sender = result_sender.clone();
                let panic_name = child_name.clone();
                std::panic::set_hook(Box::new(move |panic_info| {
                    child_result_sender
                        .lock()
                        .unwrap()
                        .take()
                        .unwrap()
                        .send(Err(Error::new(panic_info.to_string())))
                        .unwrap();
                    child_shared_state.kill(&panic_name);
                }));

                // Log that the thread has started
                debug!(
                    global_shared_state.logger(),
                    "Started thread \"{}\"", child_name
                );

                // Run the thread function
                let result = f(&global_shared_state);

                // Log the result of the thread function
                match &result {
                    Ok(()) => debug!(
                        global_shared_state.logger(),
                        "Thread \"{}\" completed successfully", child_name,
                    ),
                    Err(error) => error!(
                        global_shared_state.logger(),
                        "Thread \"{}\" ended with error: {}", child_name, error
                    ),
                }

                // Send the result back to the main thread and kill the program
                result_sender
                    .lock()
                    .unwrap()
                    .take()
                    .unwrap()
                    .send(result)
                    .unwrap();
                global_shared_state.kill(&child_name);

                #[allow(unused_must_use)]
                std::panic::take_hook();
            })
            .map_err(|error| {
                Error::new_with(format!("unable to spawn \"{}\" thread", name), error)
            })?;

        Ok(Thread {
            join_handle,
            result: result_receiver,
            on_kill: Box::new(on_kill),
            name,
        })
    }
}
