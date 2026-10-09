use crate::{SingleValueSender, threads::single_value_channel::SingleValueSharedState};
use std::sync::Arc;

impl<T> SingleValueSender<T> {
    /// Create a new [`SingleValueSender`]
    pub(in crate::threads::single_value_channel) fn new(
        shared_state: Arc<SingleValueSharedState<T>>,
    ) -> SingleValueSender<T> {
        SingleValueSender { shared_state }
    }
}
