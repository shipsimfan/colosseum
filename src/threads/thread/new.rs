use crate::{
    Error, GlobalSharedState, Result, debug, error,
    threads::{Thread, single_value_channel},
};
use std::sync::Arc;

impl Thread {
    /// Create a new thread with the provided name
    pub fn new<
        F1: 'static + FnOnce(&GlobalSharedState) -> Result<()> + Send,
        F2: 'static + FnOnce(),
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
                debug!(
                    global_shared_state.logger(),
                    "Started thread \"{}\"", child_name
                );

                let result = f(&global_shared_state);

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

                result_sender.send(result).unwrap();
                global_shared_state.kill();
            })
            .map_err(|error| {
                Error::new_with(format!("unable to spawn \"{}\" thread", name), error)
            })?;

        Ok(Thread {
            join_handle,
            result: result_receiver,
            on_kill: Box::new(on_kill),
        })
    }
}
