// 这个文件故意无法通过编译，用于观察 E0040。
// 运行：rustc compile_fail/call_drop_directly.rs

struct Resource;

impl Drop for Resource {
    fn drop(&mut self) {
        println!("释放资源");
    }
}

fn main() {
    let mut resource = Resource;

    // Drop::drop 只借用了 resource，不能表示所有权已经结束。
    // Rust 禁止直接调用析构方法，应该改用 std::mem::drop(resource)。
    Drop::drop(&mut resource);
}

