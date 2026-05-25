use crate::logging::FormatterKind;

impl std::fmt::Display for FormatterKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            FormatterKind::None => "none",
            FormatterKind::Human => "human",
            FormatterKind::Json => "json",
            FormatterKind::JsonPretty => "json-pretty",
        }
        .fmt(f)
    }
}
