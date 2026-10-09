use crate::{Error, SingleValueReceiver};

impl<T> SingleValueReceiver<T> {
    /// Take the value from the channel, if it is available
    pub fn try_take(self) -> Result<T, SingleValueReceiver<T>> {
        if self.is_available() {
            let value = unsafe { self.shared_state.value.get().as_mut() }.unwrap();
            Ok(value.take().unwrap())
        } else {
            Err(self)
        }
    }

    /// Take the value from the channel
    ///
    /// # Panics
    /// Panics if the value is not available. Use [`SingleValueReceiver::is_available`] to check
    /// if the value is available before calling this method or use
    /// [`SingleValueReceiver::try_take`] to attempt to take the value without panicking.
    pub fn take(self) -> T {
        self.try_take()
            .ok()
            .expect("value is not available to be taken")
    }

    /// Wait until the value is available without taking it from the channel
    pub fn wait_no_take(&self) -> Result<(), Error> {
        self.shared_state
            .notify
            .as_ref()
            .unwrap()
            .wait(None)
            .map_err(Error::new_inner)?;
        Ok(())
    }

    /// Wait until the value is available and then take it from the channel
    pub fn wait(self) -> Result<T, Error> {
        self.wait_no_take()?;
        Ok(self.take())
    }
}
