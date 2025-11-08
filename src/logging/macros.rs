/// Log `message` to `logger` with `severity`
#[macro_export]
macro_rules! log {
    ($severity: expr, $logger: expr, $($arg: tt)*) => {
        if $logger.should_log($severity) {
            $logger.log($severity, ::std::format!($($arg)*), ::std::module_path!());
        }
    };
}

/// Log `message` to `logger` as an error
#[macro_export]
macro_rules! error {
    ($logger: expr, $($arg: tt)*) => {
        $crate::log!($crate::logging::LogSeverity::Error, $logger, $($arg)*)
    };
}

/// Log `message` to `logger` as an warning
#[macro_export]
macro_rules! warning {
    ($logger: expr, $($arg: tt)*) => {
        $crate::log!($crate::logging::LogSeverity::Warning, $logger, $($arg)*)
    };
}

/// Log `message` to `logger` as an information message
#[macro_export]
macro_rules! info {
    ($logger: expr, $($arg: tt)*) => {
        $crate::log!($crate::logging::LogSeverity::Info, $logger, $($arg)*)
    };
}

/// Log `message` to `logger` as an debug message
#[macro_export]
macro_rules! debug {
    ($logger: expr, $($arg: tt)*) => {
        $crate::log!($crate::logging::LogSeverity::Debug, $logger, $($arg)*)
    };
}
