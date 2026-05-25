use crate::{
    Error, GlobalSharedState, Result, debug, error,
    threads::{Thread, thread::ThreadSharedState},
};
use std::sync::Arc;

impl Thread {
    /// Create a new thread with the provided name
    pub fn new<F: 'static + FnOnce(&GlobalSharedState) -> Result<()> + Send>(
        name: String,
        f: F,
        global_shared_state: Arc<GlobalSharedState>,
    ) -> Result<Thread> {
        let thread_shared_state = Arc::new(ThreadSharedState::new(name.clone()));
        let child_shared_state = thread_shared_state.clone();

        let join_handle = std::thread::Builder::new()
            .name(name.clone())
            .spawn(move || {
                debug!(
                    global_shared_state.logger(),
                    "Started thread \"{}\"",
                    child_shared_state.name()
                );

                let result = f(&global_shared_state);

                match &result {
                    Ok(()) => debug!(
                        global_shared_state.logger(),
                        "Thread \"{}\" completed successfully",
                        child_shared_state.name(),
                    ),
                    Err(error) => error!(
                        global_shared_state.logger(),
                        "Thread \"{}\" ended with error: {}",
                        child_shared_state.name(),
                        error
                    ),
                }

                child_shared_state.kill(result);
                global_shared_state.kill();
            })
            .map_err(|error| {
                Error::new_with(format!("unable to spawn \"{}\" thread", name), error)
            })?;

        Ok(Thread {
            join_handle,
            shared_state: thread_shared_state,
        })
    }
}
