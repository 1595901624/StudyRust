use std::cell::RefCell;
use std::rc::Rc;
use std::sync::{Arc, Mutex};
use std::sync::mpsc::channel;
use std::thread;

fn main() {
    // let rc = Rc::new(1);
    // let rc = Arc::new(1);
    let rc = Arc::new(Mutex::new(RefCell::new(1)));
    let a = rc.clone();
    thread::spawn(move || {
        a.clone();
    }).join().unwrap();
}
