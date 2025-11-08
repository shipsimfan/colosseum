use crate::logging::FormatterKind;

#[cfg(debug_assertions)]
impl FormatterKind {
    /// The default formatter to use for standard output
    pub(in crate::logging::options) const STDOUT_DEFAULT: FormatterKind = FormatterKind::Human;

    /// The default formatter to use for combined log file
    pub(in crate::logging::options) const COMBINED_FILE_DEFAULT: FormatterKind =
        FormatterKind::JsonPretty;

    /// The default formatter to use for scoped log files
    pub(in crate::logging::options) const SCOPED_FILES_DEFAULT: FormatterKind =
        FormatterKind::Human;
}

#[cfg(not(debug_assertions))]
impl FormatterKind {
    /// The default formatter to use for standard output
    pub(in crate::logging::options) const STDOUT_DEFAULT: FormatterKind = FormatterKind::None;

    /// The default formatter to use for combined log file
    pub(in crate::logging::options) const COMBINED_FILE_DEFAULT: FormatterKind =
        FormatterKind::Json;

    /// The default formatter to use for scoped log files
    pub(in crate::logging::options) const SCOPED_FILES_DEFAULT: FormatterKind = FormatterKind::None;
}
