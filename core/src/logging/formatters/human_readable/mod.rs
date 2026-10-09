mod formatter;
mod new;

/// A formatter which outputs human readable messages
#[derive(Clone)]
pub(in crate::logging) struct HumanReadableFormatter {
    /// Should the output include color?
    color: bool,
}
