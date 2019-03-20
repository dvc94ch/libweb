/// Outputs an error message to stdout.
#[macro_export]
macro_rules! console_log {
    ($($arg: tt)*) => (
        println!($($arg)*);
    )
}

/// Outputs an error message to stderr.
#[macro_export]
macro_rules! console_error {
    ($($arg: tt)*) => (
        eprintln!($($arg)*)
    )
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_console_log() {
        let world = "world";
        console_log!("hello\n{}", world);
    }

    #[test]
    fn test_console_error() {
        let world = "world";
        console_error!("hello\n{}", world);
    }
}
