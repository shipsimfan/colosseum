use crate::render::{Assets, LoadingMesh};
use std::path::PathBuf;

impl Assets {
    /// Load a mesh into the assets
    pub fn load_mesh<P: Into<PathBuf>>(&self, path: P) -> LoadingMesh {
        let path = path.into();

        let name = match path.file_name() {
            Some(name) => name.to_string_lossy().into_owned(),
            None => format!("unnamed_mesh"),
        };

        let op = self.file_io.read_mesh_file(path);

        LoadingMesh::new_reading(op, name)
    }
}
