use crate::{SingleValueReceiver, threads::single_value_channel::SingleValueSharedState};
use std::sync::Arc;

impl<T> SingleValueReceiver<T> {
    /// Create a new [`SingleValueReceiver`]
    pub(in crate::threads::single_value_channel) fn new(
        shared_state: Arc<SingleValueSharedState<T>>,
    ) -> SingleValueReceiver<T> {
        SingleValueReceiver { shared_state }
    }
}
