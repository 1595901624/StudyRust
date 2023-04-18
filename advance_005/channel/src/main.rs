use std::sync::mpsc::channel;

fn main() {
    println!("Hello, world!");

    let (sender,receiver) = channel();
    sender.send("hello").unwrap();

    dbg!(receiver.recv()).unwrap();
}
