use crate::threads::single_value_channel::SingleValueSharedState;
use std::sync::Arc;

mod is_available;
mod new;
mod take;

/// A receiver for a single value channel. This can be used to receive a single value from another
/// thread
pub(crate) struct SingleValueReceiver<T> {
    /// The shared state between the sender and receiver
    shared_state: Arc<SingleValueSharedState<T>>,
}
