use std::sync::mpsc::channel;
use std::thread;
use std::thread::sleep;
use std::time::Duration;

fn main() {
    let (sender, receiver) = channel::<String>();

    let handle1 = thread::spawn(move || {
        sleep(Duration::from_millis(1000));
        let hello = "hello".to_string();
        let world = "world".to_string();
        // 发送者的所有权转移至线程内,发送 hello 字符串
        sender.send(hello).unwrap();
        sender.send(world).unwrap();
        // if let Err(error) = sender.send("hello".to_string()) {
        //     println!("发送错误 {:?}", error);
        // }
    });

    let handle2 = thread::spawn(move || {
        // 接收者接收 hello 字符串
        // let receive_hello = receiver.recv().unwrap();
        // println!("receive_hello: {}", receive_hello);

        // 迭代
        receiver.into_iter().for_each(|item| {
            println!("receive_hello: {}", item);
        });
    });

    handle1.join().unwrap();
    handle2.join().unwrap();
}
