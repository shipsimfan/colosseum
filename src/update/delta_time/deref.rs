use crate::update::DeltaTime;
use std::{ops::Deref, time::Duration};

impl Deref for DeltaTime {
    type Target = Duration;

    fn deref(&self) -> &Self::Target {
        &self.delta_time
    }
}

impl AsRef<Duration> for DeltaTime {
    fn as_ref(&self) -> &Duration {
        &self.delta_time
    }
}
