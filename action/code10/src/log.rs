#![allow(unused)]

use std::fs::OpenOptions;
use std::io::Write;

use chrono::Local;
use serde::{Deserialize, Serialize};

#[derive(Debug, PartialOrd, Ord, Eq, PartialEq, Copy, Clone, Serialize, Deserialize)]
pub enum LogLevel {
    Info,
    Debug,
    Warning,
    Error,
    Fatal,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Logger {
    // 日志文件路径
    log_file: Option<String>,
    // 日志输出的级别
    log_level: LogLevel,
    // 是否写日志到文件
    enable_write_file: bool,
    // 是否打印到控制台
    enable_print_console: bool,
}

/// 默认日志配置
impl Default for Logger {
    fn default() -> Self {
        Logger {
            log_file: None,
            log_level: LogLevel::Info,
            enable_write_file: false,
            enable_print_console: true,
        }
    }
}


impl Logger {
    pub fn new(log_file: Option<String>,
               log_level: LogLevel,
               enable_write_file: bool,
               enable_print_console: bool) -> Self {
        Logger {
            log_file,
            log_level,
            enable_write_file,
            enable_print_console,
        }
    }

    /// 打印日志
    pub fn log(&self, message: &str, log_level: LogLevel) {
        // 根据日志级别判断是否需要打印
        if log_level >= self.log_level {
            let log_string = format!(
                "{} [{}] {}",
                self.get_current_time(),
                self.get_log_level_string(log_level),
                message
            );
            // 是否开启写文件
            if self.enable_write_file {
                match &self.log_file {
                    Some(file) => self.write_to_file(&log_string, file),
                    None => println!("{} [ERROR => file path is empty!]", log_string),
                }
            }

            // 是否开启打印到控制台
            if self.enable_print_console {
                println!("{}", log_string);
            }
        }
    }

    pub fn d(&self, message: &str) {
        self.log(message, LogLevel::Debug);
    }

    // pub fn i(&self, message: &str) {
    //     self.log(message, LogLevel::Info);
    // }

    pub fn i<M>(&self, message: M) where M: AsRef<str> {
        self.log(message.as_ref(), LogLevel::Info);
    }

    pub fn w(&self, message: &str) {
        self.log(message, LogLevel::Warning);
    }

    pub fn e(&self, message: &str) {
        self.log(message, LogLevel::Error);
    }

    pub fn f(&self, message: &str) {
        self.log(message, LogLevel::Fatal);
    }

    /// 获取当前时间
    fn get_current_time(&self) -> String {
        let current_time = Local::now();
        return current_time.format("%Y-%m-%d %H:%M:%S%.3f").to_string();
    }

    fn get_log_level_string(&self, log_level: LogLevel) -> &str {
        match log_level {
            LogLevel::Info => "INFO",
            LogLevel::Warning => "WARNING",
            LogLevel::Error => "ERROR",
            LogLevel::Debug => "DEBUG",
            LogLevel::Fatal => "FATAL",
        }
    }

    fn write_to_file(&self, log_string: &str, file: &str) {
        if let Ok(mut file) = OpenOptions::new().create(true).append(true).open(file) {
            if let Err(err) = writeln!(&mut file, "{}", log_string) {
                eprintln!("Failed to write to log file: {}", err);
            }
        } else {
            eprintln!("Failed to open log file");
        }
    }
}