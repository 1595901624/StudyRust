use std::sync::Arc;
use std::thread;
use std::time::Duration;

fn main() {
    // 1. 线程的创建与使用
    // create_thread();

    // 2. 跨线程共享"不可变数据"
    // shared_data();

    join_handle();
}

fn join_handle() {
    let duration = Duration::from_millis(1000);
    let handle = thread::spawn(move || {
        thread::sleep(duration);
        // 返回一个 String 类型
        return "hello".to_string();
    });
    let handle_finished = handle.is_finished();
    println!("handle_finished: {}", handle_finished);

    let thread = handle.thread();
    println!("{:?}", thread);

    // 父线程接收子线程运行结束的结果
    let result = handle.join().unwrap();

    println!("{}", result);
}

/// 跨线程共享"不可变数据"
fn shared_data() {
    let duration = Duration::from_millis(1000);
    // 这样可以解决问题，但在数据量很大的时候，不推荐
    // let data = vec!["hello".to_string(), "rust".to_string()];

    // Arc 原子引用计数
    let data: Arc<Vec<String>> = Arc::new(vec!["hello".to_string(), "rust".to_string()]);

    // clone 仅会增加引用计数，不会真正的 clone 数据
    let handle1_data = data.clone();
    let handle1 = thread::spawn(move || {
        thread::sleep(duration);
        println!("线程1 {:?}", handle1_data);
        println!("线程1 执行完成");
    });

    // clone 仅会增加引用计数，不会真正的 clone 数据
    let handle2_data = data.clone();
    let handle2 = thread::spawn(move || {
        thread::sleep(duration);
        println!("线程2 {:?}", handle2_data);
        println!("线程2 执行完成");
    });

    // 等待线程结束
    handle1.join().unwrap();
    handle2.join().unwrap();

    println!("程序 执行结束");
}

/// 线程的创建与使用
fn create_thread() {
    let duration = Duration::from_millis(3000);
    let handle1 = thread::spawn(move || {
        thread::sleep(duration);
        println!("线程1 执行完成");
    });

    let handle2 = thread::spawn(move || {
        thread::sleep(duration);
        println!("线程2 执行完成");
    });

    // 等待线程结束
    handle1.join().unwrap();
    handle2.join().unwrap();

    println!("程序 执行结束");
}
