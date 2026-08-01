/// 模拟一个需要显式释放的底层句柄
struct Handle {
    id: u32,
}

impl Drop for Handle {
    fn drop(&mut self) {
        println!("2. 释放底层句柄 {}", self.id);
    }
}

struct Connection {
    name: String,
    // 使用 Option 包装，析构时可以通过 take 留下一个有效的 None
    handle: Option<Handle>,
}

impl Drop for Connection {
    fn drop(&mut self) {
        if let Some(handle) = self.handle.take() {
            println!("1. 关闭连接 {}", self.name);

            // 提前析构取出的句柄方法返回后，drop glue 只会看到 None，
            // 因此不会再次释放同一个 Handle
            drop(handle);
        }
    }
}

pub fn run() {
    let _connection = Connection {
        name: String::from("cache"),
        handle: Some(Handle { id: 21 }),
    };

    // _connection 离开作用域时，会进入上面的 Drop::drop
}
