use std::sync::{Arc, RwLock};
use std::thread;
use std::time::Duration;

fn main() {
    // 使用`Arc`和`RwLock`创建一个共享数据结构
    let shared_data = Arc::new(RwLock::new(0));

    // 创建多个读线程
    for _ in 0..5 {
        let shared_data = Arc::clone(&shared_data);
        thread::spawn(move || {
            // 获取读锁
            let reader = shared_data.read().unwrap();

            // 在共享数据上执行只读操作
            println!("读取: {}", *reader);

            // 当`reader`离开作用域时，读锁会被释放
        });
    }

    // 创建一个写线程
    thread::spawn(move || {
        // 在获取写锁之前延迟2秒
        thread::sleep(Duration::from_secs(2));

        // 获取写锁
        let mut writer = shared_data.write().unwrap();

        // 在共享数据上执行写操作
        *writer += 1;
        println!("写入: {}", *writer);

        // 当`writer`离开作用域时，写锁会被释放
    });

    // 等待所有线程执行完毕
    thread::sleep(Duration::from_secs(5));
}