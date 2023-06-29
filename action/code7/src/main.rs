mod test;

use std::net::{IpAddr, SocketAddr};
use std::thread;
use std::time::Duration;

use tokio::net::TcpStream;
use futures::{stream, StreamExt};

const PORTS: &[u16] = &[21, 22, 23, 25, 80, 110, 139, 443, 3306, 3389, 8080, 8081, 8888, 22122, 65535];

#[tokio::main]
async fn main() {
    // scan_ports_by_sequential().await;
    scan_ports_by_concurrency().await;
}

async fn scan_ports_by_concurrency() {
    let ip_string = "182.61.200.6";
    let ip = ip_string.parse::<IpAddr>().unwrap();
    let ports = stream::iter(PORTS.into_iter());
    let x = ports.for_each_concurrent(10, |port| {
        scan_single_port(ip, *port)
    });
    x.await;
}

async fn scan_ports_by_sequential() {
    let ip_string = "182.61.200.6";
    let ip = ip_string.parse::<IpAddr>().unwrap();
    // 循环扫描单个端口
    for x in PORTS {
        println!("正在扫描端口: {}", x);
        scan_single_port(ip, *x).await;
        println!("端口 {}已经扫描完成", x);
    }
}

async fn scan_single_port(target: IpAddr, port: u16) {
    let address = SocketAddr::new(target, port);
    // println!("正在扫描:{}", address);
    let future = TcpStream::connect(address);
    if let Ok(Ok(_)) = tokio::time::timeout(Duration::from_secs(5), future).await {
        println!("开放端口：{}", port);
    }
}

// #[tokio::main]
// async fn main() -> Result<(), Box<dyn std::error::Error>> {
//     use tokio::net::TcpListener;
//     use tokio::io::{AsyncReadExt, AsyncWriteExt};
//
//     let listener = TcpListener::bind("127.0.0.1:8080").await?;
//
//     loop {
//         let (mut socket, _) = listener.accept().await?;
//
//         tokio::spawn(async move {
//             let mut buf = [0; 1024];
//
//             loop {
//                 let n = match socket.read(&mut buf).await {
//                     Ok(n) if n == 0 => return,
//                     Ok(n) => n,
//                     Err(e) => {
//                         println!("发生错误 {:?}", e);
//                         return;
//                     }
//                 };
//
//                 match socket.write_all(&buf[0..n]).await {
//                     Ok(_) => {
//                         println!("我被访问了!");
//                     }
//                     Err(e) => {
//                         println!("发生错误 {:?}", e);
//                         return;
//                     }
//                 }
//             }
//         });
//     }
// }