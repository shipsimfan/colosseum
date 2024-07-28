use super::Command;
use crate::logging::{log_to_console, NAME};
use alexandria::Severity;
use std::{
    borrow::Cow,
    fs::{File, OpenOptions},
    io::Write,
    path::{Path, PathBuf},
    sync::mpsc::Receiver,
    time::{Duration, SystemTime, UNIX_EPOCH},
};

/// A currently open file, referenced by one or more loggers
struct OpenFile {
    /// The file itself
    file: File,

    /// The number of loggers referencing this file
    count: usize,
}

/// The main entry point for the log writer thread
pub(super) fn run(queue: Receiver<Command>, directory: PathBuf) {
    let mut files = Vec::new();

    while let Ok(command) = queue.recv() {
        match command {
            Command::Open(name) => open_file(name, &directory, &mut files),
            Command::Write(name, severity, message, time) => {
                if let Err(error) = write(name, severity, message, time, &mut files) {
                    log_to_console(
                        Severity::Error,
                        NAME,
                        &format_args!("Failed while writing for {} - {}", name, error),
                    );
                }
            }
            Command::Close(name) => close_file(name, &mut files),
        }
    }
}

/// Opens a file or increases its reference count
fn open_file(name: &'static str, directory: &Path, files: &mut Vec<(&'static str, OpenFile)>) {
    for open_file in files.iter_mut() {
        if open_file.0 == name {
            open_file.1.count += 1;
            return;
        }
    }

    let path = Path::new(directory).join(format!("{}.log", name));
    let file = match OpenOptions::new().append(true).create(true).open(&path) {
        Ok(file) => file,
        Err(error) => {
            log_to_console(
                Severity::Error,
                NAME,
                &format_args!("Failed to open \"{}\" - {}", path.display(), error),
            );
            return;
        }
    };

    files.push((name, OpenFile { file, count: 1 }));
}

/// Closes an open file or decrements its reference count
fn close_file(name: &'static str, files: &mut Vec<(&'static str, OpenFile)>) {
    let mut file = None;
    for (i, open_file) in files.iter_mut().enumerate() {
        if open_file.0 == name {
            file = Some((i, &mut open_file.1));
            break;
        }
    }

    if file.is_none() {
        return;
    }
    let (i, file) = file.unwrap();

    file.count -= 1;
    if file.count == 0 {
        files.swap_remove(i);
    }
}

/// Writes the log message to the approriate file
fn write(
    name: &'static str,
    severity: Severity,
    message: Cow<'static, str>,
    time: SystemTime,
    files: &mut Vec<(&'static str, OpenFile)>,
) -> std::io::Result<()> {
    let mut file = None;
    for open_file in files.iter_mut() {
        if open_file.0 == name {
            file = Some(&mut open_file.1);
            break;
        }
    }

    if file.is_none() {
        return Ok(());
    }
    let file = file.unwrap();

    write!(file.file, "[{}][", severity)?;
    write_time(&mut file.file, time, 0)?;
    writeln!(file.file, "] {}", message)
}

/// Writes `system_time` to `output` as an ISO 8601 date-time
fn write_time(output: &mut File, timestamp: SystemTime, offset: i16) -> std::io::Result<()> {
    let total_millis = timestamp
        .duration_since(UNIX_EPOCH)
        .unwrap_or(Duration::ZERO)
        .as_millis();

    let milli = (total_millis % 1_000) as u16;
    let total_seconds = (total_millis / 1_000) as u64;

    let second = (total_seconds % 60) as u8;
    let total_minutes = total_seconds / 60;

    let minute = (total_minutes % 60) as u8;
    let total_hours = total_minutes / 60;

    let hour = (total_hours % 24) as u8;
    let total_days = total_hours / 24;

    let days_since_0 = total_days + 719468;
    let era = days_since_0 / 146097;
    let doe = days_since_0 - (era * 146097);
    let yoe = (doe - (doe / 1460) + (doe / 36524) - (doe / 146096)) / 365;
    let year = yoe + era * 400;
    let doy = doe - (365 * yoe + (yoe / 4) - (yoe / 400));
    let mp = ((5 * doy) + 2) / 153;
    let day = doy - ((153 * mp) + 2) / 5 + 1;
    let month = if mp < 10 { mp + 3 } else { mp - 9 };

    write!(
        output,
        "{:04}-{:02}-{:02}T{:02}:{:02}:{:02}.{:03}",
        year, month, day, hour, minute, second, milli
    )?;
    if offset == 0 {
        write!(output, "Z")
    } else {
        let offset_hour = offset / 60;
        let offset_minute = (offset % 60).abs();

        write!(output, "{:02}:{:02}", offset_hour, offset_minute)
    }
}
