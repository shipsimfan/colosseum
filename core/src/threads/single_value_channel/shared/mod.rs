use alexandria::Notify;
use std::{cell::UnsafeCell, sync::atomic::AtomicBool};

mod new;

/// The state shared between the sender and receiver of a single value channel
pub(in crate::threads::single_value_channel) struct SingleValueSharedState<T> {
    /// A boolean to track if the value has been sent or not
    pub sent: AtomicBool,

    /// The value being sent
    pub value: UnsafeCell<Option<T>>,

    /// A notify to wake the receiver when the value is sent
    pub notify: Option<Notify>,
}

unsafe impl<T> Send for SingleValueSharedState<T> where T: Send {}
unsafe impl<T> Sync for SingleValueSharedState<T> where T: Send {}
