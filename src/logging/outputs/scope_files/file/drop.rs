use crate::logging::outputs::scope_files::ScopeFile;

impl<Formatter: crate::logging::Formatter> Drop for ScopeFile<Formatter> {
    fn drop(&mut self) {
        self.formatter.end(&mut self.file).ok();
    }
}
