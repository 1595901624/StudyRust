use std::ffi::{c_int, c_void};

fn main() {
    println!("Hello, world!");
    let a: std::os::raw::c_short = 3;
    let b: std::ffi::c_short = 4;
    println!("{}", a);
    println!("{}", b);

    unsafe {
        let ptr = malloc(1 * std::mem::size_of::<i32>());
        let iptr = ptr as *mut i32;
        iptr.write(15);
        println!("{}", *iptr);
        println!("{:?}", ptr);
        free(ptr);

        let b = -5;
        let c = abs(b);
        println!("{}", c);

        let d = getch();
        println!("{}", d);

    }
}

// 声明 C 标准库函数 `malloc`
extern "C" {
    fn malloc(size: usize) -> *mut c_void;

    fn free(ptr: *mut c_void);

    fn abs(i: c_int) -> c_int;

    fn getch() -> c_int;
}