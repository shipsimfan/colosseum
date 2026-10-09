use crate::{Result, new_error, single_value_channel::SingleValueSharedState};
use alexandria::Notify;
use std::{cell::UnsafeCell, sync::atomic::AtomicBool};

impl<T> SingleValueSharedState<T> {
    /// Create a new shared state for a single value channel
    pub fn new(notify: bool) -> Result<SingleValueSharedState<T>> {
        let notify = if notify {
            Some(Notify::new(false, false).map_err(|error| {
                new_error!("unable to create a single value channel - {}", error)
            })?)
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
