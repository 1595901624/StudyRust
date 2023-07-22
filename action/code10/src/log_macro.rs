/// 通过配置文件创建 Logger
#[macro_export]
macro_rules! log_default {
    () => {
        {
            let cargo = env!("CARGO_MANIFEST_DIR");
            let config_path = Path::new(cargo).join("src").join("config").join("log_config.json");
            let config_string = std::fs::read_to_string(config_path);
            if let Ok(path) = config_string {
                if let Ok(logger) = serde_json::from_str(&path) {
                    logger
                } else {
                    Logger::default()
                }
            } else {
                Logger::default()
            }
        }
    };
}

/// 打印 info 日志
#[macro_export]
macro_rules! info {
    ($arg:expr) => {
        let logger = log_default!();
        logger.i($arg);
    };
}

/// 打印 debug 日志
#[macro_export]
macro_rules! debug {
    ($arg:expr) => {
        let logger = log_default!();
        logger.d($arg);
    };
}

/// 打印 error 日志
#[macro_export]
macro_rules! error {
    ($arg:expr) => {
        let logger = log_default!();
        logger.e($arg);
    };
}

/// 打印 warn 日志
#[macro_export]
macro_rules! warn {
    ($arg:expr) => {
        let logger = log_default!();
        logger.w($arg);
    };
}

/// 打印 fatal 日志
#[macro_export]
macro_rules! fatal {
    ($arg:expr) => {
        let logger = log_default!();
        logger.f($arg);
    };
}


