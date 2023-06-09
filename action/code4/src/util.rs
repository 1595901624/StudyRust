use std::process::Command;
use std::time::Duration;
use crossterm::event::{Event, KeyCode, poll, read};

/// 清空屏幕
pub fn clear_screen() {
    if cfg!(target_os = "windows") {
        Command::new("cmd.exe").arg("/c").arg("cls").status().expect("clear error!");
    } else if cfg!(target_os = "linux") {
        Command::new("clear").status().unwrap();
    }
}

/// 获取键盘按键
pub fn keyboard_hit() -> Option<KeyCode> {
    // 等待 0 秒，立即返回。
    match poll(Duration::from_millis(0)) {
        Ok(true) => match read() {
            // 如果存在按键，则返回键盘按键
            Ok(Event::Key(key)) => {
                // println!("{:?}", key);
                return Some(key.code);
            } // 根据需要修改触发按键
            _ => None,
        },
        _ => {
            None
        }
    }

}