/// Log `message` to `logger` with `severity`
#[macro_export]
macro_rules! log {
    ($severity: ident, $logger: expr, $($arg: tt)*) => {
        if $logger.should_log($crate::logging::LogSeverity::$severity) {
            $logger.log(
                $crate::logging::LogSeverity::$severity,
                ::std::format!($($arg)*),
                ::std::module_path!()
            );
        }
    };
}

/// Log `message` to `logger` as an error
#[macro_export]
macro_rules! error {
    ($logger: expr, $($arg: tt)*) => {
        $crate::log!(Error, $logger, $($arg)*)
    };
}

/// Log `message` to `logger` as an warning
#[macro_export]
macro_rules! warning {
    ($logger: expr, $($arg: tt)*) => {
        $crate::log!(Warning, $logger, $($arg)*)
    };
}

/// Log `message` to `logger` as an information message
#[macro_export]
macro_rules! info {
    ($logger: expr, $($arg: tt)*) => {
        $crate::log!(Info, $logger, $($arg)*)
    };
}

/// Log `message` to `logger` as an debug message
#[macro_export]
macro_rules! debug {
    ($logger: expr, $($arg: tt)*) => {
        $crate::log!(Debug, $logger, $($arg)*)
    };
}
