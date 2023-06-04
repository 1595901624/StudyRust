use std::sync::{Arc, Mutex, MutexGuard};
use std::sync::mpsc::channel;
use std::thread;
use std::thread::sleep;
use std::time::Duration;

fn main() {
    // test_mutex();

    test_mpmc();
}


/// 2.mpmc
fn test_mpmc() {
    let (sender, receiver) = channel::<i32>();

    // 创建多接收者
    let multi_receiver = Arc::new(Mutex::new(receiver));

    let sender1 = sender.clone();
    let sender_handle1 = thread::spawn(move || {
        sender1.send(5).unwrap();
    });

    let sender_handle2 = thread::spawn(move || {
        sleep(Duration::from_millis(1000));
        sender.send(10).unwrap();
    });


    let receiver1 = multi_receiver.clone();
    let receiver_handle1 = thread::spawn(move || loop {
        let mutex_guard = receiver1.lock().unwrap();
        if let Ok(m) = mutex_guard.recv() {
            println!("receiver1: {}", m);
            // 手动 drop，运行其它线程处理数据
            drop(mutex_guard);
            sleep(Duration::from_millis(500));
        } else {
            println!("close receiver1");
            break;
        }
    });

    let receiver2 = multi_receiver.clone();
    let receiver_handle2 = thread::spawn(move || loop {
        let mutex_guard = receiver2.lock().unwrap();
        if let Ok(m) = mutex_guard.recv() {
            println!("receiver2: {}", m);
            // 手动 drop，运行其它线程处理数据
            drop(mutex_guard);
            sleep(Duration::from_millis(500));
        } else {
            println!("close receiver2");
            break;
        }
    });

    receiver_handle1.join().unwrap();
    receiver_handle2.join().unwrap();
    sender_handle1.join().unwrap();
    sender_handle2.join().unwrap();
}

/// 1.mutex
fn test_mutex() {
    // 创建一个字符串
    let hello = Arc::new(Mutex::new(String::from("hello")));
    // 创建向量保存接下来的10个线程
    let mut handles = Vec::with_capacity(10);
    for i in 0..10 {
        let temp = hello.clone();
        // 创建线程
        let handle = thread::spawn(move || {
            // 在子线程中，修改字符串的值
            let mut x: MutexGuard<String> = temp.lock().unwrap();
            x.push(char::from(65 + i));
        });
        handles.push(handle);
    }

    // 执行所有子线程
    for handle in handles {
        handle.join().unwrap();
    }

    // 打印结果
    let result = hello.lock().unwrap();
    println!("{:?}", *result);
}
