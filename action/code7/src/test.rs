#[test]
pub fn test_thread() {
    use std::thread;
    use std::time::Duration;

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

#[test]
pub fn test_channel() {
    use std::sync::mpsc;
    use std::thread;

    let (tx, rx) = mpsc::channel();

    let handle1 = thread::spawn(move || {
        let val = String::from("hi");
        tx.send(val).unwrap();
        println!("发送完毕");
    });

    let handle2 = thread::spawn(move || {
        let received = rx.recv().unwrap();
        println!("收到: {}", received);
    });


    handle1.join().unwrap();
    handle2.join().unwrap();
}

#[test]
pub fn test_mutex() {
    use std::sync::Mutex;
    use std::thread;
    use std::sync::{Arc, MutexGuard};

    // 创建一个字符串
    let lanqiao = Arc::new(Mutex::new(String::from("LANQIAO")));
    // 创建向量保存接下来的10个线程
    let mut handles = Vec::with_capacity(10);
    for i in 0..10 {
        let temp = lanqiao.clone();
        // 创建线程
        let handle = thread::spawn(move || {
            // 在子线程中，修改字符串的值
            let mut x: MutexGuard<String> = temp.lock().unwrap();
            x.push(char::from(97 + i));
        });
        handles.push(handle);
    }

    // 执行所有子线程
    for handle in handles {
        handle.join().unwrap();
    }

    // 打印结果
    let result = lanqiao.lock().unwrap();
    println!("{:?}", result);
}

#[test]
fn test_future() {
    use std::future::Future;
    use std::pin::Pin;
    use std::task::{Context, Poll};
    use futures::FutureExt;

    struct MyFuture {
        count: usize,
    }

    impl Future for MyFuture {
        type Output = usize;

        fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
            if self.count == 0 {
                Poll::Ready(self.count)
            } else {
                println!("count: {}", self.count);
                self.count -= 1;
                cx.waker().wake_by_ref();
                Poll::Pending
            }
        }
    }

    let mut future = MyFuture { count: 10 };
    let mut cx = Context::from_waker(futures::task::noop_waker_ref());
    while let Poll::Pending = future.poll_unpin(&mut cx) {
        println!("polling");
    }
}