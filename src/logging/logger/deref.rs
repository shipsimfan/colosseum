use crate::logging::{LogController, Logger};
use std::{ops::Deref, sync::Arc};

impl Deref for Logger {
    type Target = Arc<LogController>;

    fn deref(&self) -> &Self::Target {
        &self.controller
    }
}
