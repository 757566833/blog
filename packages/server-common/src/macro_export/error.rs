

#[macro_export]
macro_rules! macro_log_error {
    ($err:expr) => {
        ::tracing::error!(
            file = file!(),
            line = line!(),
            error = %$err,
            "An error occurred"
        );
    };
    ($err:expr, $msg:literal $(, $args:expr)* ) => {
        ::tracing::error!(
            file = file!(),
            line = line!(),
            error = %$err,
            $msg $(, $args)*
        );
    };
}


#[macro_export]
macro_rules! macro_panic_log_error {
    // 没有自定义消息，只打印默认
    ($file:expr, $line:expr, $err:expr) => {
        ::tracing::error!(
            file = $file,
            line = $line,
            error = %$err,
            "An error occurred"
        );
    };

    // 有自定义消息和可变参数
    ($file:expr, $line:expr, $err:expr, $msg:literal $(, $args:expr)* ) => {
        ::tracing::error!(
            file = $file,
            line = $line,
            error = %$err,
            $msg $(, $args)*
        );
    };
}