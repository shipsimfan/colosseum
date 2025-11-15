use crate::{info, logging::Logger};
use std::{arch::x86_64::__cpuid, path::PathBuf};
use time::DateTime;

#[cfg(debug_assertions)]
const BUILD_TYPE: &str = "Debug";

#[cfg(not(debug_assertions))]
const BUILD_TYPE: &str = "Release";

#[cfg(target_arch = "x86_64")]
const CPU_ARCHITECTURE: &str = "x86-64";

/// Logs the metadata about the program and the system
pub(in crate::run) fn log_metadata<Game: crate::Game>(logger: &Logger, start_time: DateTime) {
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
        "Company: {}{}, Build Type: {}",
        Game::COMPANY,
        match option_env!("COLOSSEUM_GAME_COMMIT") {
            Some(commit) => format!(", Game Commit: #{}", commit),
            None => String::new(),
        },
        BUILD_TYPE,
    );
    if let Some(game_build_time) = option_env!("COLOSSEUM_GAME_BUILD_TIME") {
        info!(logger, "Game Build Time: {}", game_build_time);
    }

    // Log engine info
    info!(
        logger,
        "Engine: {} v{}, Engine Commit: #{}",
        env!("CARGO_PKG_NAME"),
        env!("CARGO_PKG_VERSION"),
        env!("COLOSSEUM_ENGINE_COMMIT")
    );
    info!(
        logger,
        "Engine Build Time: {}",
        env!("COLOSSEUM_ENGINE_BUILD_TIME")
    );

    // Log operating system info

    // Log CPU
    info!(
        logger,
        "CPU Architecture: {}, CPU Model: {}, Cores: {}, Threads: {}",
        CPU_ARCHITECTURE,
        get_cpu_model(),
        get_cpu_cores(),
        get_cpu_threads()
    );

    // Log system language
}

fn get_cpu_model() -> String {
    // Get the highest extended result
    let highest_extended_result = unsafe { __cpuid(0x80000000) }.eax;

    // If doesn't support extended brand, fallback to legacy
    if highest_extended_result < 0x8000004 {
        let legacy_result = unsafe { __cpuid(0) };
        let ebx = legacy_result.ebx.to_le_bytes();
        let ecx = legacy_result.ecx.to_le_bytes();
        let edx = legacy_result.edx.to_le_bytes();
        let bytes = [
            ebx[0], ebx[1], ebx[2], ebx[3], edx[0], edx[1], edx[2], edx[3], ecx[0], ecx[1], ecx[2],
            ecx[3],
        ];
        let mut length = 0;
        for byte in bytes {
            if byte == 0 {
                break;
            }

            length += 1;
        }
        return String::from_utf8_lossy(&bytes[..length]).to_string();
    }

    // Pull extended brand name
    let brand_name1 = unsafe { __cpuid(0x80000002) };
    let eax1 = brand_name1.eax.to_le_bytes();
    let ebx1 = brand_name1.ebx.to_le_bytes();
    let ecx1 = brand_name1.ecx.to_le_bytes();
    let edx1 = brand_name1.edx.to_le_bytes();

    let brand_name2 = unsafe { __cpuid(0x80000003) };
    let eax2 = brand_name2.eax.to_le_bytes();
    let ebx2 = brand_name2.ebx.to_le_bytes();
    let ecx2 = brand_name2.ecx.to_le_bytes();
    let edx2 = brand_name2.edx.to_le_bytes();

    let brand_name3 = unsafe { __cpuid(0x80000004) };
    let eax3 = brand_name3.eax.to_le_bytes();
    let ebx3 = brand_name3.ebx.to_le_bytes();
    let ecx3 = brand_name3.ecx.to_le_bytes();
    let edx3 = brand_name3.edx.to_le_bytes();

    let bytes = [
        eax1[0], eax1[1], eax1[2], eax1[3], ebx1[0], ebx1[1], ebx1[2], ebx1[3], ecx1[0], ecx1[1],
        ecx1[2], ecx1[3], edx1[0], edx1[1], edx1[2], edx1[3], eax2[0], eax2[1], eax2[2], eax2[3],
        ebx2[0], ebx2[1], ebx2[2], ebx2[3], ecx2[0], ecx2[1], ecx2[2], ecx2[3], edx2[0], edx2[1],
        edx2[2], edx2[3], eax3[0], eax3[1], eax3[2], eax3[3], ebx3[0], ebx3[1], ebx3[2], ebx3[3],
        ecx3[0], ecx3[1], ecx3[2], ecx3[3], edx3[0], edx3[1], edx3[2], edx3[3],
    ];

    let mut length = 0;
    for byte in bytes {
        if byte == 0 {
            break;
        }

        length += 1;
    }
    return String::from_utf8_lossy(&bytes[..length]).to_string();
}

fn get_cpu_cores() -> u32 {
    0
}

fn get_cpu_threads() -> u32 {
    0
}
