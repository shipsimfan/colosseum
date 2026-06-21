use crate::{Error, Result, info, logging::Logger};
use std::path::PathBuf;
use time::DateTime;

#[cfg(debug_assertions)]
const BUILD_TYPE: &str = "Debug";

#[cfg(not(debug_assertions))]
const BUILD_TYPE: &str = "Release";

/// Logs the metadata about the program and the system
pub(in crate::run) fn log_metadata<Game: crate::Game>(
    logger: &Logger,
    start_time: DateTime,
    game_branch: Option<&str>,
    game_hash: Option<&str>,
    game_build_time: Option<&str>,
) -> Result<()> {
    // Log starting
    info!(logger, "Starting {} v{} . . .", Game::NAME, Game::VERSION);
    info!(logger, "Start Time: {}", start_time.iso8601());

    // Log working directory
    info!(
        logger,
        "Working directory: {}",
        std::env::current_dir().unwrap_or(PathBuf::new()).display()
    );

    // Log command arguments
    let mut arguments = String::new();
    let mut first = true;
    for argument in std::env::args_os() {
        if first {
            first = false;
        } else {
            arguments.push(' ');
        }
        arguments.push('"');
        arguments.push_str(&argument.to_string_lossy());
        arguments.push('"');
    }
    info!(logger, "Command line args: {}", arguments);

    // Log game info
    info!(
        logger,
        "Company: {}{}{}, Build Type: {}",
        Game::COMPANY,
        match game_branch {
            Some(branch) => format!(", Game Branch: #{}", branch),
            None => String::new(),
        },
        match game_hash {
            Some(commit) => format!(", Game Commit: #{}", commit),
            None => String::new(),
        },
        BUILD_TYPE,
    );
    if let Some(game_build_time) = game_build_time {
        info!(logger, "Game Build Time: {}", game_build_time);
    }

    // Log engine info
    info!(
        logger,
        "Engine: {} v{}, Engine Branch: {}, Engine Commit: #{}",
        env!("CARGO_PKG_NAME"),
        env!("CARGO_PKG_VERSION"),
        env!("COLOSSEUM_ENGINE_BRANCH"),
        env!("COLOSSEUM_ENGINE_COMMIT")
    );
    info!(
        logger,
        "Engine Build Time: {}",
        env!("COLOSSEUM_ENGINE_BUILD_TIME")
    );

    // Log operating system info
    info!(
        logger,
        "Operating System: {} ({})",
        alexandria::system::os_name(),
        alexandria::system::os_version().map_err(|error| Error::new_inner(error.to_string()))?
    );

    // Log CPU
    info!(
        logger,
        "CPU Architecture: {}, CPU Model: {}, Cores: {}",
        alexandria::system::Architecture::CURRENT,
        alexandria::system::cpu_model(),
        alexandria::system::cpu_cores(),
    );

    // Log total memory
    info!(
        logger,
        "Installed Memory: {}",
        alexandria::system::installed_memory()
    );

    Ok(())
}
