use crate::logging::HumanReadableFormatter;

impl HumanReadableFormatter {
    /// Create a new [`HumanReadableFormatter`]
    pub fn new(color: bool) -> Self {
        HumanReadableFormatter { color }
    }
}
