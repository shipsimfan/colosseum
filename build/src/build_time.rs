/// Pushes the current build time as "COLOSSEUM_GAME_BUILD_TIME"
pub fn push_game_build_time() {
    push_build_time("COLOSSEUM_GAME_BUILD_TIME");
}

/// Pushes the current build time as `variable`
pub fn push_build_time(variable: &str) {
    println!("cargo:rustc-env={}={}", variable, get_build_time());
}

/// Gets the current build time
pub fn get_build_time() -> String {
    time::DateTime::<time::SimpleTimeZone>::now_local()
        .iso8601()
        .to_string()
}
