/// Log `message` to `logger` with `severity`
#[macro_export]
macro_rules! logger {
    ($scope: literal) => {
        $crate::LogController::get().logger($scope)
    };
}

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
        $crate::log!($crate::LogSeverity::Error, $logger, $($arg)*)
    };
}

/// Log `message` to `logger` as an warning
#[macro_export]
macro_rules! warning {
    ($logger: expr, $($arg: tt)*) => {
        $crate::log!($crate::LogSeverity::Warning, $logger, $($arg)*)
    };
}

/// Log `message` to `logger` as an information message
#[macro_export]
macro_rules! info {
    ($logger: expr, $($arg: tt)*) => {
        $crate::log!($crate::LogSeverity::Info, $logger, $($arg)*)
    };
}

/// Log `message` to `logger` as an debug message
#[macro_export]
macro_rules! debug {
    ($logger: expr, $($arg: tt)*) => {
        $crate::log!($crate::LogSeverity::Debug, $logger, $($arg)*)
    };
}

/// Log `message` with `severity` and `scope`
#[macro_export]
macro_rules! log_s {
    ($severity: expr, $scope: literal, $($arg: tt)*) => {{
        let log_controller = $crate::LogController::get();

        if log_controller.should_log($severity) {
            log_controller.log($severity, ::std::format!($($arg)*), $scope, ::std::module_path!());
        }
    }};
}

/// Log `message` as an error with `scope`
#[macro_export]
macro_rules! error_s {
    ($scope: literal, $($arg: tt)*) => {
        $crate::log_s!($crate::LogSeverity::Error, $scope, $($arg)*)
    };
}

/// Log `message` as an warning with `scope`
#[macro_export]
macro_rules! warning_s {
    ($scope: literal, $($arg: tt)*) => {
        $crate::log_s!($crate::LogSeverity::Warning, $scope, $($arg)*)
    };
}

/// Log `message` as an information message with `scope`
#[macro_export]
macro_rules! info_s {
    ($scope: literal, $($arg: tt)*) => {
        $crate::log_s!($crate::LogSeverity::Info, $scope, $($arg)*)
    };
}

/// Log `message` as an debug message with `scope`
#[macro_export]
macro_rules! debug_s {
    ($scope: literal, $($arg: tt)*) => {
        $crate::log_s!($crate::LogSeverity::Debug, $scope, $($arg)*)
    };
}
