use crate::logging::JsonFormatter;

impl Clone for JsonFormatter {
    fn clone(&self) -> Self {
        JsonFormatter::new(self.pretty)
    }
}
