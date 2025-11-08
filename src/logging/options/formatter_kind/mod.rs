mod default;
mod display;
mod flag;

/// The type of formatter that can be assigned to an output
#[derive(PartialEq, Eq)]
pub enum FormatterKind {
    /// Don't use the output
    None,

    /// Display human readable output
    Human,

    /// Output JSON
    Json,

    /// Output pretty JSON
    JsonPretty,
}
