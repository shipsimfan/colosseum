use crate::logging::JsonFormatter;

impl JsonFormatter {
    /// Create a new [`JsonFormatter`]
    pub fn new(pretty: bool) -> Self {
        JsonFormatter {
            pretty,
            first: true,
        }
    }
}
