use std::io::{Read, Write};
use std::net::{SocketAddr, TcpListener};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // 指定要绑定的地址和端口号
    let addr = "192.168.1.107:9988".parse::<SocketAddr>()?;

    // 创建TCP Listener并将其绑定到指定的地址上
    let listener = TcpListener::bind(&addr)?;

    println!("Server listening on {}", addr);

    loop {
        match listener.accept() {
            Ok((stream, _)) => {
                handle_connection(stream);
            }
            Err(_) => break,
        };
    }
    Ok(())
}

/// 处理连接
fn handle_connection(mut stream: impl Read + Write) {
    // 读取客户端发送过来的消息
    let mut buffer = [0u8; 4096];
    if let Ok(n) = stream.read(&mut buffer[..]) {
        // 打印收到的消息内容
        print!("Received message from client: ");
        for byte in &buffer[..n] {
            print!("{} ", *byte as char);
        }

        // 向客户端返回响应消息
        let response = b"Hello HarmonyOS from server!";
        stream.write_all(response).unwrap();
    } else {
        eprintln!("Failed to read data");
    }
}