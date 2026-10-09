use crate::ThreadManager;

impl<UserData: 'static + Send> Drop for ThreadManager<UserData> {
    fn drop(&mut self) {
        self.kill("ThreadManager drop").unwrap();
    }
}
