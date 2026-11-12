use crate::{Logger, debug, file_io::ReadMeshFileOp, warning};
use colosseum_core::new_error;

impl ReadMeshFileOp {
    /// Execute the file I/O operation
    pub(in crate::file_io::operations) fn execute(self, logger: &Logger) {
        self.result
            .send(
                parthenon::Mesh::load(&self.path)
                    .map(|mesh| {
                        debug!(logger, "Read mesh from \"{}\"", self.path.display());
                        mesh
                    })
                    .map_err(|error| {
                        warning!(
                            logger,
                            "Unable to read from \"{}\" - {}",
                            self.path.display(),
                            error
                        );
                        new_error!(
                            "unable to read from \"{}\" - {}",
                            self.path.display(),
                            error,
                        )
                    }),
            )
            .unwrap();
    }
}
