/// 手动释放资源
struct Resource;

impl Drop for Resource {
    fn drop(&mut self) {
        println!("release");
    }
}

pub fn run() {
    let mut resource = Resource;
    // 禁止调用 Drop::drop 方法
    // Drop::drop(&mut resource);
    // 手动调用 std::mem::drop 方法来释放资源
    std::mem::drop(resource);
}
