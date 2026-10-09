use crate::threads::single_value_channel::SingleValueSharedState;
use std::sync::Arc;

mod new;
mod send;

/// A sender for a single value channel. This can be used to send a single value to another thread
pub(crate) struct SingleValueSender<T> {
    /// The shared state between the sender and receiver
    shared_state: Arc<SingleValueSharedState<T>>,
}
