use crate::{Error, Result, threads::single_value_channel::SingleValueSharedState};
use alexandria::Notify;
use std::{cell::UnsafeCell, sync::atomic::AtomicBool};

impl<T> SingleValueSharedState<T> {
    /// Create a new shared state for a single value channel
    pub fn new(notify: bool) -> Result<SingleValueSharedState<T>> {
        let notify = if notify {
            Some(Notify::new(false, false).map_err(Error::new_inner)?)
        } else {
            None
        };

        Ok(SingleValueSharedState {
            sent: AtomicBool::new(false),
            value: UnsafeCell::new(None),
            notify,
        })
    }
}
