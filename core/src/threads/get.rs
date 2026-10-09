use crate::ThreadManager;

impl<UserEvent: 'static + Send> ThreadManager<UserEvent> {
    /// Is the system currently running?
    pub fn is_running(&self) -> bool {
        self.shared_state.is_running()
    }
}
