use std::ffi::{c_char, c_int, c_void, CStr, CString};

fn main() {
    unsafe {
        // 调用 C 语言的 malloc 函数分配足够存储一个 i32 类型值的内存空间
        let ptr = malloc(1 * std::mem::size_of::<i32>());
        // 将从 malloc 返回的 void* 类型裸指针（*mut std::ffi::c_void）强制转换为 *mut i32 类型的裸指针
        // 这个指针指向 i32 类型的值
        let i32_ptr = ptr as *mut i32;
        // 使用 .write() 方法将数字 15 写入到之前分配的内存中。
        i32_ptr.write(15);
        // 输出
        println!("i32ptr 的地址 {:?}, i32_ptr 的值 {:?}", i32_ptr, i32_ptr.read());
        // 释放之前由 malloc 分配的内存。
        free(ptr);

        // 调用 C 语言的 abs 函数求绝对值
        let b = -5;
        println!("b = {}, abs(b) = {:?}", b, abs(b));

        // 调用 C 语言的 getchar 函数获取键盘输入
        let d = getchar();
        // 由于 Windows 中的 c_char 类型是 i8，所以需要先将其转换为 u8 类型，再转为 char
        println!("输入的键盘字符：{}", d as u8 as char);


        // C 语言中的结构体
        let name = CString::new("张三").unwrap();
        let student = Student { id: 1, name: name.as_ptr() };
        println!("student = id: {}, name: {:?}", student.id, CStr::from_ptr(student.name).to_str());
    }
}

#[repr(C)]
struct Student {
    // 学号
    pub id: c_int,
    // 姓名
    pub name: *const c_char,
}

// 声明 C 标准库函数 `malloc`
extern "C" {
    // 返回一个指向 `size` 大小的内存区域的指针
    fn malloc(size: usize) -> *mut c_void;

    // 释放内存
    fn free(ptr: *mut c_void);

    // 求绝对值
    fn abs(i: c_int) -> c_int;

    // 获取键盘输入
    fn getchar() -> c_char;
}