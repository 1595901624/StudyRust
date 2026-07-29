/// 用于观察字段自己的析构逻辑。
struct Socket {
    id: u32,
}

impl Drop for Socket {
    fn drop(&mut self) {
        println!("2. 执行 Socket::drop，释放 socket {}", self.id);
    }
}

/// `Connection` 实现了 `Drop`，它的字段 `Socket` 也实现了 `Drop`。
struct Connection {
    name: String,
    socket: Socket,
}

impl Drop for Connection {
    fn drop(&mut self) {
        // 此时字段仍然有效，因此可以通过 &mut self 读取或修改字段。
        println!(
            "1. 执行 Connection::drop，连接名称为 {}，socket id 为 {}",
            self.name, self.socket.id
        );

        // 当前方法返回后，编译器生成的 drop glue 才会继续析构字段。
    }
}

pub fn run() {
    let connection = Connection {
        name: String::from("database"),
        socket: Socket { id: 7 },
    };

    println!("正在使用连接 {}", connection.name);

    // connection 在当前函数结尾离开作用域。
    // Rust 先调用 Connection::drop，再按照字段声明顺序析构字段。
}
