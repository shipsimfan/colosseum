use std::path::{Path, PathBuf};

/// Pushes the commit hash of the current working directory as the environment variable
/// "COLOSSEUM_GAME_COMMIT", if there is one
pub fn push_game_commit_hash() {
    push_commit_hash("COLOSSEUM_GAME_COMMIT");
}

/// Pushes the commit hash of the current working directory as the environment `variable`, if there
/// is one
pub fn push_commit_hash(variable: &str) {
    if let Some(commit_hash) = get_commit_hash() {
        println!("cargo:rustc-env={}={}", variable, commit_hash);
    }
}

/// Gets the commit hash of the current working directory, if there is one
pub fn get_commit_hash() -> Option<String> {
    let git_repo = get_git_repo_path()?;

    let head = std::fs::read_to_string(git_repo.join("HEAD")).expect("cannot read git repo `HEAD`");
    let ref_path = match head.strip_prefix("ref: ") {
        Some(ref_path) => ref_path.trim(),
        None => return Some(head),
    };

    Some(std::fs::read_to_string(git_repo.join(ref_path)).expect("cannot read branch ref"))
}

/// Get the path to the root of the git repo
fn get_git_repo_path() -> Option<PathBuf> {
    let base_path = Path::new(".git");
    if !base_path.exists() {
        return None;
    }

    if base_path.is_dir() {
        return Some(base_path.to_path_buf());
    }

    let contents = std::fs::read_to_string(base_path).expect("cannot read `.git` file");
    let path = contents
        .strip_prefix("gitdir: ")
        .expect("invalid `.git` file")
        .trim();

    Some(PathBuf::from(path))
}
