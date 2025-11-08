use crate::logging::CombinedFileOutput;

impl<Formatter: crate::logging::Formatter> Drop for CombinedFileOutput<Formatter> {
    fn drop(&mut self) {
        self.formatter.end(&mut self.file).ok();
    }
}
