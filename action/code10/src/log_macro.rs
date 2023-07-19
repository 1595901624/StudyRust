#[macro_export]
macro_rules! info {
    ($arg:expr) => {
        let logger: log::Logger = log::Logger::new(Some(String::from("log.txt")), log::LogLevel::Info, true, true);
        logger.i($arg);
    };
}