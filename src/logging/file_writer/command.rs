use alexandria::Severity;
use std::{borrow::Cow, time::SystemTime};

/// A command sent to the log writer thread
pub(super) enum Command {
    /// Open a log file (or increase it's reference count)
    Open(&'static str),

    /// Close a log file (or decrease it's reference count)
    Close(&'static str),

    /// Write a message to a log file
    Write(&'static str, Severity, Cow<'static, str>, SystemTime),
}
