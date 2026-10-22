use std::{ffi::OsStr, path::Path};

const SHADER_DIRS: &[&str] = &["examples", "src"];

fn main() {
    colosseum_build::push_branch("COLOSSEUM_ENGINE_BRANCH");
    colosseum_build::push_commit_hash("COLOSSEUM_ENGINE_COMMIT");
    colosseum_build::push_build_time("COLOSSEUM_ENGINE_BUILD_TIME");

    let shader_extension = OsStr::new("slang");
    for path in SHADER_DIRS {
        search_for_files(path, shader_extension);
    }
}

fn search_for_files<P: AsRef<Path>>(path: P, extension: &OsStr) {
    for entry in std::fs::read_dir(path).unwrap() {
        let entry = entry.unwrap();
        let path = entry.path();

        if entry.file_type().unwrap().is_dir() {
            search_for_files(path, extension);
            continue;
        }

        if path.extension().unwrap_or(OsStr::new("")) == extension {
            println!("cargo:rerun-if-changed={}", path.display());
        }
    }
}
