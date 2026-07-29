struct Resource {
    name: &'static str,
}

impl Drop for Resource {
    fn drop(&mut self) {
        println!("释放资源 {}", self.name);
    }
}

pub fn run() {
    let resource = Resource { name: "log-file" };

    println!("资源仍然可以使用：{}", resource.name);

    // std::mem::drop 按值取得 resource 的所有权。
    // 函数结束时会运行完整析构流程，后面不能继续使用 resource。
    drop(resource);

    println!("resource 已经提前析构");

    // 取消下一行的注释会得到 E0382，因为 resource 的所有权已经移动。
    // println!("{}", resource.name);
}
