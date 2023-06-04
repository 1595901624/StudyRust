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

    /// 增加秒数
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

/// 格式化字符
#[test]
fn test_format() {
    // 1. 默认
    println!("hello world!");
    // 2. 通配符
    println!("今天是 {} 年 {} 月 {} 日", 2023, 6, 22);
    // 3. 通配符 + 位置下标
    println!("{0} * {1} = {2}, {2} / {1} = {0}", 5, 6, 30);
    // 4. 通配符 + 命名参数
    // 4.1
    println!("我的名字叫{name}, 今年{age}岁, 喜欢{hobby}", hobby = "打篮球", name = "张三", age = 18);
    // 4.2
    let name = "Lisi";
    let hobby = "唱歌";
    let age = 21;
    println!("我的名字叫{name}, 今年{age}岁, 喜欢{hobby}");
    // 5. 通配符 + 对齐方式  + 对齐宽度
    println!("二进制 {:b}", 31);
    println!("八进制 {:o}", 31);
    println!("十六进制(小写) {:x}", 31);
    println!("十六进制(大写) {:X}", 31);
    println!("科学计数(小写) {:e}", 100000_f32);
    println!("科学计数(大写) {:E}", 100000_f32);
    println!("输出指针  {:p}", "rust");
    println!("打印Debug视图 {:?}", ["A", "B", "C", "D"]);
    println!("输出标点符号 {:+}", 5);
    // 6 其它符号
    // 6.1 #
    println!("带前缀符二进制 {:#b}", 31);
    println!("带前缀符八进制 {:#o}", 31);
    println!("带前缀符十六进制(小写) {:#x}", 31);
    println!("带前缀符十六进制(大写) {:#X}", 31);
    println!("带换行和缩进的Debug打印 {:#?}", ["A", "B", "C", "D"]);

    // 6.2  > < ^
    println!("使用大于号右对齐 {:>6}{:>6}{:>6}", 1, 2, 3);
    println!("使用小于号左对齐 {:<6}{:<6}{:<6}", 1, 2, 3);
    println!("省略大于号右对齐 {:6}{:6}{:6}", 1, 2, 3);
    println!("居中对齐 {:^6}{:^6}{:^6}", 1, 2, 3);
    println!("填充任意字符居中对齐 {:-^6}{:*^6}{:1^6}", 1, 2, 3);

    // 6.3 0
    println!("二进制8位补零 {:08b}", 31);
    println!("八进制8位补零 {:08o}", 31);
    println!("十六进制16位补零 {:016b}", 31);

    //6.4
    println!("小数保留位数 {:.3} ", 0.01);
    println!("{}小数保留3位数 {:.*} --- 保留4位数 {:.*} ", 0.01, 3, 0.01, 4, 0.10);
}

fn main() {}