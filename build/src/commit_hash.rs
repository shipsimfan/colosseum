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
    alexandria::git::current_commit_hash()
}
