use crate::logging::outputs::scope_files::ScopeFile;
use std::path::Path;

impl<Formatter: crate::logging::Formatter> ScopeFile<Formatter> {
    /// Open a file for `scope` under `base_path`
    pub fn open(
        base_path: &Path,
        scope: &'static str,
        mut formatter: Formatter,
    ) -> std::io::Result<Self> {
        let path = base_path.join(format!("{}.{}", scope, Formatter::EXTENSION));
        let mut file = std::fs::OpenOptions::new()
            .write(true)
            .create(true)
            .truncate(true)
            .open(&path)?;

        formatter.start(&mut file)?;

        Ok(ScopeFile {
            scope,
            path,
            file,
            formatter,
        })
    }
}
