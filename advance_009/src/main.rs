use std::mem::ManuallyDrop;
use std::sync::atomic;
use std::sync::atomic::{AtomicBool, AtomicI32, AtomicI8, AtomicPtr};

fn main() {
    // 1. load/store
    load_store();
    // 2. compare_and_swap
    compare_and_swap();
    // 3. fetch_add/sub
    fetch_add_sub();
    // 4. AtomicBool
    atomic_bool();
    // 5. AtomicPtr
    atomic_ptr();
    // 6. fetch_update
    fetch_update();
    // 7. fetch_and/or/xor/nand
    fetch_and_or_xor_nand();
}

/// load/store
fn load_store() {
    println!("==================== load/store ====================");
    let i = AtomicI8::new(5);
    let m = i.load(atomic::Ordering::SeqCst);
    println!("i 的初始值 = {}", m);
    i.store(10, atomic::Ordering::SeqCst);
    let n = i.load(atomic::Ordering::SeqCst);
    println!("i 修改后的值 = {}", n);
}

/// compare_and_swap
fn compare_and_swap() {
    println!("==================== compare_and_swap ====================");
    let i = AtomicI8::new(5);
    let m = i.load(atomic::Ordering::SeqCst);
    println!("i 的初始值 = {}", m);
    // 1.50.0 不再推荐使用 compare_and_swap，推荐使用 compare_exchange/compare_exchange_weak
    // let x = i.compare_and_swap(5, 10, atomic::Ordering::SeqCst);
    let x = i.compare_exchange(5, 10, atomic::Ordering::SeqCst, atomic::Ordering::SeqCst);
    let n = i.load(atomic::Ordering::SeqCst);
    println!("i 修改后的值 = {}", n);
    println!("i compare_and_swap 后的返回值 = {:?}", x);
}

/// fetch_add/sub
fn fetch_add_sub() {
    println!("==================== fetch_add/sub ====================");
    let i = AtomicI8::new(5);
    let m = i.load(atomic::Ordering::SeqCst);
    println!("i 的初始值 = {}", m);
    let x = i.fetch_add(10, atomic::Ordering::SeqCst);
    let n = i.load(atomic::Ordering::SeqCst);
    println!("i fetch_add(10) 后的值 = {}", n);
    println!("i fetch_add(10) 后的返回值 = {}", x);
    let x = i.fetch_sub(10, atomic::Ordering::SeqCst);
    let n = i.load(atomic::Ordering::SeqCst);
    println!("i fetch_sub(10) 后的值 = {}", n);
    println!("i fetch_sub(10) 后的返回值 = {}", x);
}

/// AtomicBool
fn atomic_bool() {
    println!("==================== AtomicBool ====================");
    let b = AtomicBool::new(true);
    let m = b.load(atomic::Ordering::SeqCst);
    println!("b 的初始值 = {}", m);
    b.store(false, atomic::Ordering::SeqCst);
    let n = b.load(atomic::Ordering::SeqCst);
    println!("b 修改后的值 = {}", n);
}

/// AtomicPtr
fn atomic_ptr() {
    println!("==================== AtomicPtr ====================");
    let mut hello_str = "hello".to_string();
    let p = AtomicPtr::new(&mut hello_str);
    let mut rust_str = "rust".to_string();
    p.store(&mut rust_str, atomic::Ordering::SeqCst);
    let c = p.load(atomic::Ordering::SeqCst);
    // 使用 ManuallyDrop 来避免双重释放
    let result = unsafe { ManuallyDrop::new(std::ptr::read(c)) };
    println!("p 修改后的值 = {}", *result);
}

/// fetch_update
fn fetch_update() {
    println!("==================== fetch_update ====================");
    let i = AtomicI8::new(5);
    let m = i.load(atomic::Ordering::SeqCst);
    println!("i 的初始值 = {}", m);
    let x = i.fetch_update(atomic::Ordering::SeqCst, atomic::Ordering::SeqCst, |x| {
        if x == 5 {
            Some(10)
        } else {
            None
        }
    });
    let n = i.load(atomic::Ordering::SeqCst);
    println!("i fetch_update 后的值 = {}", n);
    println!("i fetch_update 后的返回值 = {:?}", x);
}

/// fetch_and/or/xor/nand
fn fetch_and_or_xor_nand() {
    println!("==================== fetch_and/or/xor/nand ====================");
    let i = AtomicI32::new(5);
    let m = i.load(atomic::Ordering::SeqCst);
    println!("i 的初始值 = {}", m);
    let x = i.fetch_and(10, atomic::Ordering::SeqCst);
    let n = i.load(atomic::Ordering::SeqCst);
    println!("i fetch_and(10) 后的值 = {}", n);
    println!("i fetch_and(10) 后的返回值 = {}", x);
    let x = i.fetch_or(10, atomic::Ordering::SeqCst);
    let n = i.load(atomic::Ordering::SeqCst);
    println!("i fetch_or(10) 后的值 = {}", n);
    println!("i fetch_or(10) 后的返回值 = {}", x);
    let x = i.fetch_xor(10, atomic::Ordering::SeqCst);
    let n = i.load(atomic::Ordering::SeqCst);
    println!("i fetch_xor(10) 后的值 = {}", n);
    println!("i fetch_xor(10) 后的返回值 = {}", x);
    let x = i.fetch_nand(10, atomic::Ordering::SeqCst);
    let n = i.load(atomic::Ordering::SeqCst);
    println!("i fetch_nand(10) 后的值 = {}", n);
    println!("i fetch_nand(10) 后的返回值 = {}", x);
}