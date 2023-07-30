use std::sync::{Arc, RwLock};
use std::thread;
use std::time::Duration;

fn main() {
    // lock1();
    lock2();
}

/// 使用读写锁
pub fn lock1() {
    let rust = "hello rust!".to_string();
    let lock = RwLock::new(rust);
    {
        let read1 = lock.read().unwrap();
        let read2 = lock.read().unwrap();
        println!("read1: {}, read2: {}", read1, read2);
    } // 读锁在这里被释放

    // 仅能持有一个写锁
    {
        let mut write = lock.write().unwrap();
        (*write).push_str(" hello world!");
        println!("write: {}", *write);
    } // 写锁在这里被释放
}

/// 线程中使用读写锁
pub fn lock2() {
    // 使用`Arc`和`RwLock`创建一个共享数据结构
    let shared_data = Arc::new(RwLock::new(0));

    // 创建读线程
    let reader_shared_data = shared_data.clone();
    thread::spawn(move || {
        // 获取读锁
        let reader = reader_shared_data.read().unwrap();

        // 在共享数据上执行只读操作
        println!("读取: {}", *reader);

        // 当`reader`离开作用域时，读锁会被释放
    }).join().unwrap();

    // 创建一个写线程
    let writer_data = shared_data.clone();
    thread::spawn(move || {
        // 获取写锁
        let mut writer = writer_data.write().unwrap();

        // 在共享数据上执行写操作
        *writer += 1;
        println!("写入: {}", *writer);
        thread::sleep(Duration::from_secs(2));
        // 当`writer`离开作用域时，写锁会被释放
    }).join().unwrap();


    // 再次创建读线程
    thread::spawn(move || {
        // 获取读锁
        let reader = shared_data.read().unwrap();

        // 在共享数据上执行只读操作
        println!("读取: {}", *reader);

        // 当`reader`离开作用域时，读锁会被释放
    }).join().unwrap();
}