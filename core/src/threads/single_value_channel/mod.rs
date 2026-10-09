use crate::Result;
use shared::*;
use std::sync::Arc;

mod receiver;
mod sender;
mod shared;

pub(crate) use receiver::*;
pub(crate) use sender::*;

/// Create a channel that can be used to send a single value from one thread to another
pub fn create<T>(notify: bool) -> Result<(SingleValueSender<T>, SingleValueReceiver<T>)> {
    let shared_state = Arc::new(SingleValueSharedState::new(notify)?);

    Ok((
        SingleValueSender::new(shared_state.clone()),
        SingleValueReceiver::new(shared_state),
    ))
}
