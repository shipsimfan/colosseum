mod clone;
mod formatter;
mod new;

/// A formatter which outputs JSON
pub(in crate::logging) struct JsonFormatter {
    /// Should the JSON be pretty printed?
    pretty: bool,

    /// Is the next message going to be the first message printed?
    first: bool,
}
