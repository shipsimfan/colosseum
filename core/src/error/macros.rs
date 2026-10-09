/// Create a new error
#[macro_export]
macro_rules! new_error {
    ($message: expr) => {
        $crate::Error::new($message, ::std::file!(), ::std::line!())
    };

    ($message: literal, $($arg:tt)+) => {
        $crate::Error::new(::std::format!($message, $($arg)+), ::std::file!(), ::std::line!())
    };
}
