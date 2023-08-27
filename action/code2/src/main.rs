use std::fmt::{Display, Formatter, Write};
use std::process::Command;
use std::thread::sleep;
use std::time::Duration;
use crate::HourFormat::{Twelve, TwentyFour};

/// 数字时钟结构体
struct Clock {
    hours: u8,
    minutes: u8,
    seconds: u8,
    format: HourFormat,
}

/// 制式枚举
#[derive(Eq, PartialEq)]
enum HourFormat {
    TwentyFour,
    Twelve,
}

/// 实现 Display trait
impl Display for Clock {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        // 以下三种写法都可以达到想要的效果
        // f.write_str(format!("{:02}:{:02}:{:02}", self.hours, self.minutes, self.seconds).as_str())
        // f.write_fmt(format_args!("{:02}:{:02}:{:02}", self.hours, self.minutes, self.seconds))
        // 使用 write!宏 等同于 f.write_fmt
        write!(f, "{:02}:{:02}:{:02}", self.hours, self.minutes, self.seconds)
    }
}

/// 实现 Default trait
impl Default for Clock {
    fn default() -> Self {
        Self {
            hours: 0,
            minutes: 0,
            seconds: 0,
            format: HourFormat::TwentyFour,
        }
    }
}

impl Clock {
    fn new(hours: u8, minutes: u8, seconds: u8, format: HourFormat) -> Clock {
        Self {
            hours,
            minutes,
            seconds,
            format,
        }
    }

    /// 增加秒数
    fn add_seconds(&mut self, seconds: u8) {
        self.seconds += seconds;
        if self.seconds >= 60 {
            self.seconds -= 60;
            self.add_minutes(1);
        }
    }

    /// 增加分钟数
    fn add_minutes(&mut self, minutes: u8) {
        self.minutes += minutes;
        if self.minutes >= 60 {
            self.minutes -= 60;
            self.add_hours(1);
        }
    }

    /// 增加小时数
    fn add_hours(&mut self, hours: u8) {
        if self.format == TwentyFour {
            self.hours = (self.hours + hours) % 24;
        } else {
            let temp_hours = self.hours + hours;

            if temp_hours <= 12 {
                self.hours = temp_hours;
            } else {
                self.hours = temp_hours - 12;
            }
        }
    }

    /// 显示时间
    fn display_time(&self) -> String {
        format!("{:02}:{:02}:{:02}", self.hours, self.minutes, self.seconds)
    }
}

/// display trait
#[test]
fn test_display_trait() {
    let mut clock = Clock::new(20, 54, 40, HourFormat::TwentyFour);
    loop {
        // 清屏
        clear();
        // 自定义 Display trait 打印时间
        println!("{}", clock);
        // 自定义时间打印方法
        // println!("{}", clock.display_time());
        // 增加 秒
        clock.add_seconds(1);
        // 延时
        sleep(Duration::from_secs(1));
    }
}

/// 通过 display function
#[test]
fn test_display_function() {
    let mut clock = Clock::new(20, 54, 40, HourFormat::TwentyFour);
    loop {
        // 清屏
        clear();
        // 自定义时间打印方法
        println!("{}", clock.display_time());
        // 增加 秒
        clock.add_seconds(1);
        // 延时
        sleep(Duration::from_secs(1));
    }
}

/// 清屏方法
fn clear() {
    Command::new("clear").status().unwrap();
}

fn main() {}