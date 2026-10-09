use crate::{Error, Result, SingleValueSender};
use std::sync::atomic::Ordering;

impl<T> SingleValueSender<T> {
    /// Send a value to the receiver
    pub fn send(self, value: T) -> Result<()> {
        let value_ref = unsafe { self.shared_state.value.get().as_mut() }.unwrap();
        *value_ref = Some(value);

        self.shared_state.sent.store(true, Ordering::Release);

        if let Some(notify) = &self.shared_state.notify {
            notify.notify().map_err(Error::new_inner)
        } else {
            Ok(())
        }
    }
}
