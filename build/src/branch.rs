/// Pushes the branch of the current working directory as the environment variable
/// "COLOSSEUM_GAME_BRANCH", if there is one
pub fn push_game_branch() {
    push_branch("COLOSSEUM_GAME_BRANCH");
}

/// Pushes the branch of the current working directory as the environment `variable`, if there
/// is one
pub fn push_branch(variable: &str) {
    if let Some(branch) = get_branch() {
        println!("cargo:rustc-env={}={}", variable, branch);
    }
}

/// Gets the branch of the current working directory, if there is one
pub fn get_branch() -> Option<String> {
    alexandria::git::current_branch()
}
