use crate::logging::outputs::scope_files::ScopeFile;

impl<Formatter: crate::logging::Formatter> ScopeFile<Formatter> {
    /// Get the scope of this file
    pub fn scope(&self) -> &'static str {
        self.scope
    }
}
